// lib.rs – WASM-exported chess engine API
//
// Exposes a `ChessEngine` class to JavaScript that provides:
//   • Full legal-move generation (castling, en passant, promotions)
//   • Board state queries (FEN, turn, check, checkmate, draw, stalemate)
//   • Alpha-beta minimax AI (see search.rs)
//   • SAN move history for the move log and LLM prompts

mod eval;
mod search;

use wasm_bindgen::prelude::*;

use shakmaty::{Chess, Color, EnPassantMode, File, Move, Position, Role, Square};
use shakmaty::san::San;
use shakmaty::fen::Fen;
use shakmaty::CastlingMode;

// ── Initialisation ────────────────────────────────────────────────────────────
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn role_to_char(role: Role) -> char {
    match role {
        Role::Pawn => 'p',
        Role::Knight => 'n',
        Role::Bishop => 'b',
        Role::Rook => 'r',
        Role::Queen => 'q',
        Role::King => 'k',
    }
}

fn role_from_char(c: char) -> Option<Role> {
    match c {
        'p' | 'P' => Some(Role::Pawn),
        'n' | 'N' => Some(Role::Knight),
        'b' | 'B' => Some(Role::Bishop),
        'r' | 'R' => Some(Role::Rook),
        'q' | 'Q' => Some(Role::Queen),
        'k' | 'K' => Some(Role::King),
        _ => None,
    }
}

/// For castle moves, return the king's *destination* square so the UI can
/// highlight it correctly (g1/c1 for white, g8/c8 for black).
fn castle_king_to(king: Square, rook: Square) -> Square {
    let rank = king.rank();
    if (rook.file() as u8) > (king.file() as u8) {
        Square::from_coords(File::G, rank) // kingside
    } else {
        Square::from_coords(File::C, rank) // queenside
    }
}

/// Effective "from" square shown to the UI (always the king for castles).
fn move_from(m: &Move) -> Option<Square> {
    match m {
        Move::Castle { king, .. } => Some(*king),
        _ => m.from(),
    }
}

/// Effective "to" square shown to the UI (king destination for castles).
fn move_to(m: &Move) -> Square {
    match m {
        Move::Castle { king, rook } => castle_king_to(*king, *rook),
        _ => m.to(),
    }
}

/// chess.js-compatible flag string: k/q = castle, e = en passant, c = capture,
/// p = promotion, cp = capture+promotion, n = normal.
fn move_flags(m: &Move) -> &'static str {
    match m {
        Move::Castle { king, rook } => {
            if (rook.file() as u8) > (king.file() as u8) { "k" } else { "q" }
        }
        Move::EnPassant { .. } => "e",
        Move::Normal { capture: Some(_), promotion: Some(_), .. } => "cp",
        Move::Normal { promotion: Some(_), .. } => "p",
        Move::Normal { capture: Some(_), .. } => "c",
        _ => "n",
    }
}

/// Build a SAN string with check / checkmate suffix.
fn san_with_suffix(pos: &Chess, m: &Move) -> String {
    let san = San::from_move(pos, m).to_string();
    if let Ok(after) = pos.clone().play(m) {
        if after.is_checkmate() {
            return format!("{}#", san);
        } else if after.is_check() {
            return format!("{}+", san);
        }
    }
    san
}

// ── ChessEngine ───────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct ChessEngine {
    position: Chess,
    san_history: Vec<String>,
}

#[wasm_bindgen]
impl ChessEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChessEngine {
        ChessEngine {
            position: Chess::default(),
            san_history: Vec::new(),
        }
    }

    // ── Game state ────────────────────────────────────────────────────────────

    /// Reset to the starting position.
    pub fn reset(&mut self) {
        self.position = Chess::default();
        self.san_history.clear();
    }

    /// Load a FEN string.  Returns true on success.
    pub fn load_fen(&mut self, fen: &str) -> bool {
        let parsed: Result<Fen, _> = fen.trim().parse();
        match parsed {
            Ok(f) => match f.into_position(CastlingMode::Standard) {
                Ok(pos) => {
                    self.position = pos;
                    self.san_history.clear();
                    true
                }
                Err(_) => false,
            },
            Err(_) => false,
        }
    }

    /// Return the current FEN string.
    pub fn get_fen(&self) -> String {
        shakmaty::fen::Fen::from_position(self.position.clone(), EnPassantMode::Legal).to_string()
    }

    /// Return "w" or "b".
    pub fn get_turn(&self) -> String {
        match self.position.turn() {
            Color::White => "w".to_string(),
            Color::Black => "b".to_string(),
        }
    }

    pub fn game_over(&self) -> bool {
        self.position.is_game_over()
    }

    pub fn in_check(&self) -> bool {
        self.position.is_check()
    }

    pub fn in_checkmate(&self) -> bool {
        self.position.is_checkmate()
    }

    pub fn in_stalemate(&self) -> bool {
        self.position.is_stalemate()
    }

    pub fn in_draw(&self) -> bool {
        self.position.is_insufficient_material()
            || self.position.is_stalemate()
            || self.position.halfmoves() >= 100
    }

    /// Number of moves played so far.
    pub fn move_count(&self) -> u32 {
        self.san_history.len() as u32
    }

    // ── Move queries ──────────────────────────────────────────────────────────

    /// JSON array of verbose move objects for the given square:
    /// `[{"from":"e2","to":"e4","flags":"n","san":"e4"}, ...]`
    pub fn legal_moves_for(&self, square: &str) -> String {
        let sq: Square = match square.trim().parse() {
            Ok(s) => s,
            Err(_) => return "[]".to_string(),
        };

        let moves = self.position.legal_moves();
        let mut out = Vec::new();

        for m in moves.iter() {
            if move_from(m) != Some(sq) {
                continue;
            }
            let from_str = move_from(m).map(|s| s.to_string()).unwrap_or_default();
            let to_str = move_to(m).to_string();
            let flags = move_flags(m);
            let san = san_with_suffix(&self.position, m);

            out.push(format!(
                r#"{{"from":"{from_str}","to":"{to_str}","flags":"{flags}","san":"{san}"}}"#
            ));
        }

        format!("[{}]", out.join(","))
    }

    /// JSON array of SAN strings for all current legal moves
    /// (used in the LLM prompt).
    pub fn legal_moves_san(&self) -> String {
        let moves = self.position.legal_moves();
        let sans: Vec<String> = moves
            .iter()
            .map(|m| format!("\"{}\"", san_with_suffix(&self.position, m)))
            .collect();
        format!("[{}]", sans.join(","))
    }

    // ── Making moves ─────────────────────────────────────────────────────────

    /// Apply a move given as from/to squares + optional promotion piece char
    /// (e.g. "q").  Returns a JSON result object on success, "" on failure.
    pub fn make_move(&mut self, from_sq: &str, to_sq: &str, promotion: &str) -> String {
        let from: Square = match from_sq.trim().parse() {
            Ok(s) => s,
            Err(_) => return String::new(),
        };
        let to: Square = match to_sq.trim().parse() {
            Ok(s) => s,
            Err(_) => return String::new(),
        };
        let promo: Option<Role> = promotion.chars().next().and_then(role_from_char);

        let moves = self.position.legal_moves();
        let found = moves.iter().find(|m| {
            move_from(m) == Some(from)
                && move_to(m) == to
                && (promo.is_none() || m.promotion() == promo)
        });

        match found {
            Some(m) => {
                let san = san_with_suffix(&self.position, m);
                let from_str = move_from(m).map(|s| s.to_string()).unwrap_or_default();
                let to_str = move_to(m).to_string();
                match self.position.clone().play(m) {
                    Ok(new_pos) => {
                        self.position = new_pos;
                        self.san_history.push(san.clone());
                        format!(r#"{{"from":"{from_str}","to":"{to_str}","san":"{san}"}}"#)
                    }
                    Err(_) => String::new(),
                }
            }
            None => String::new(),
        }
    }

    /// Apply a move given in SAN notation (used for AI and LLM moves).
    /// Strips trailing check/checkmate annotations before parsing.
    /// Returns a JSON result object on success, "" on failure.
    pub fn make_san_move(&mut self, san_str: &str) -> String {
        // Strip check / checkmate / annotation suffixes before parsing
        let clean: String = san_str
            .chars()
            .take_while(|&c| !matches!(c, '+' | '#' | '!' | '?'))
            .collect();

        let san: San = match clean.trim().parse() {
            Ok(s) => s,
            Err(_) => return String::new(),
        };

        match san.to_move(&self.position) {
            Ok(m) => {
                let san_out = san_with_suffix(&self.position, &m);
                let from_str = move_from(&m).map(|s| s.to_string()).unwrap_or_default();
                let to_str = move_to(&m).to_string();
                match self.position.clone().play(&m) {
                    Ok(new_pos) => {
                        self.position = new_pos;
                        self.san_history.push(san_out.clone());
                        format!(r#"{{"from":"{from_str}","to":"{to_str}","san":"{san_out}"}}"#)
                    }
                    Err(_) => String::new(),
                }
            }
            Err(_) => String::new(),
        }
    }

    // ── AI ────────────────────────────────────────────────────────────────────

    /// Run alpha-beta minimax and return the best move in SAN notation.
    pub fn best_move(&self, depth: u32) -> String {
        search::best_move_san(&self.position, depth)
    }

    // ── History & Board ───────────────────────────────────────────────────────

    /// JSON array of SAN strings for every move played so far.
    pub fn get_san_history(&self) -> String {
        let items: Vec<String> = self
            .san_history
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect();
        format!("[{}]", items.join(","))
    }

    /// Board as an 8×8 JSON array (row 0 = rank 8).
    /// Each cell is `{"type":"p","color":"w"}` or `null`.
    pub fn get_board(&self) -> String {
        let board = self.position.board();
        let mut rows: Vec<String> = Vec::with_capacity(8);

        for rank_idx in (0..8u8).rev() {
            let rank: shakmaty::Rank = rank_idx.try_into().unwrap();
            let mut cells: Vec<String> = Vec::with_capacity(8);
            for file_idx in 0..8u8 {
                let file: shakmaty::File = file_idx.try_into().unwrap();
                let sq = Square::from_coords(file, rank);
                match board.piece_at(sq) {
                    Some(p) => {
                        let t = role_to_char(p.role);
                        let c = if p.color == Color::White { 'w' } else { 'b' };
                        cells.push(format!(r#"{{"type":"{t}","color":"{c}"}}"#));
                    }
                    None => cells.push("null".to_string()),
                }
            }
            rows.push(format!("[{}]", cells.join(",")));
        }

        format!("[{}]", rows.join(","))
    }

    /// Piece at a square as `{"type":"p","color":"w"}` JSON, or `"null"`.
    pub fn get_piece_at(&self, square: &str) -> String {
        let sq: Square = match square.trim().parse() {
            Ok(s) => s,
            Err(_) => return "null".to_string(),
        };
        match self.position.board().piece_at(sq) {
            Some(p) => {
                let t = role_to_char(p.role);
                let c = if p.color == Color::White { 'w' } else { 'b' };
                format!(r#"{{"type":"{t}","color":"{c}"}}"#)
            }
            None => "null".to_string(),
        }
    }
}
