// lib.rs – WASM-exported chess engine API
//
// Exposes a `ChessEngine` class to JavaScript that provides:
//   • Full legal-move generation (castling, en passant, promotions)
//   • Board state queries (FEN, turn, check, checkmate, draw, stalemate)
//   • Alpha-beta minimax AI (see search.rs)
//   • SAN move history for the move log and LLM prompts

mod eval;
pub mod search;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use shakmaty::{Chess, Color, EnPassantMode, Move, Position, Role, Square, CastlingSide};
use shakmaty::san::{San, SanPlus};
use shakmaty::fen::Fen;
use shakmaty::CastlingMode;

// ── Constants ──────────────────────────────────────────────────────────────
const DEFAULT_SEARCH_DEPTH: u32 = 4; // default depth for best_move()
const MAX_TT_ENTRIES: usize = 100_000;

// ── Initialisation ──────────────────────────────────────────────────────────
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
fn move_from(m: &Move) -> Option<Square> {
    match m {
        Move::Castle { king, .. } => Some(*king),
        Move::Normal { from, .. } => Some(*from),
        Move::EnPassant { from, .. } => Some(*from),
        Move::Put { .. } => None,
    }
}

fn move_to(m: &Move) -> Square {
    match m {
        Move::Castle { king, rook } => {
            let rank = king.rank();
            if rook.file() > king.file() {
                Square::from_coords(shakmaty::File::G, rank) // kingside
            } else {
                Square::from_coords(shakmaty::File::C, rank) // queenside
            }
        }
        Move::Normal { to, .. } => *to,
        Move::EnPassant { to, .. } => *to,
        Move::Put { to, .. } => *to,
    }
}

/// chess.js-compatible flag string: k/q = castle, e = en passant, c = capture,
/// p = promotion, cp = capture+promotion, n = normal.
fn move_flags(m: &Move) -> &'static str {
    match m {
        Move::Castle { king, rook } => {
            if rook.file() > king.file() { "k" } else { "q" }
        }
        Move::EnPassant { .. } => "e",
        Move::Normal { capture, promotion, .. } => {
            if capture.is_some() && promotion.is_some() { "cp" }
            else if promotion.is_some() { "p" }
            else if capture.is_some() { "c" }
            else { "n" }
        }
        _ => "n",
    }
}

/// Build a SAN string with check / checkmate suffix.
fn san_with_suffix(pos: &Chess, m: &Move) -> String {
    let _san = San::from_move(pos, m);
    SanPlus::from_move(pos.clone(), m).to_string()
}

// ── ChessEngine ──────────────────────────────────────────────────────────────────

#[wasm_bindgen]
pub struct ChessEngine {
    position: Chess,
    san_history: Vec<String>,
    tt: HashMap<u64, (i32, u32)>, // pos_hash -> (eval, depth)
}

#[wasm_bindgen]
impl ChessEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ChessEngine {
        ChessEngine {
            position: Chess::default(),
            san_history: Vec::new(),
            tt: HashMap::new(),
        }
    }

    // ── Game state ────────────────────────────────────────────────────────────

    pub fn reset(&mut self) {
        self.position = Chess::default();
        self.san_history.clear();
        self.tt.clear();
    }

    pub fn load_fen(&mut self, fen: &str) -> bool {
        let parsed: Result<Fen, _> = fen.trim().parse();
        match parsed {
            Ok(f) => match f.into_position(CastlingMode::Standard) {
                Ok(pos) => {
                    self.position = pos;
                    self.san_history.clear();
                    self.tt.clear();
                    true
                }
                Err(_) => false,
            },
            Err(_) => false,
        }
    }

    pub fn get_fen(&self) -> String {
        shakmaty::fen::Fen::from_position(self.position.clone(), EnPassantMode::Legal).to_string()
    }

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

    pub fn move_count(&self) -> u32 {
        self.san_history.len() as u32
    }

    pub fn get_evaluation(&self) -> i32 {
        eval::evaluate(&self.position)
    }

    // ── Board representation for UI ───────────────────────────────────────────

    /// Returns a JSON-serialized 8x8 array of piece objects.
    /// `[[{"type":"r","color":"w"}, ...], ...]`
    pub fn get_board(&self) -> String {
        let mut board_json = Vec::new();
        let board = self.position.board();
        
        for rank in (0..8).rev() {
            let mut row = Vec::new();
            for file in 0..8 {
                let sq = Square::from_coords(
                    shakmaty::File::new(file as u32),
                    shakmaty::Rank::new(rank as u32)
                );
                match board.piece_at(sq) {
                    Some(piece) => {
                        let type_char = role_to_char(piece.role);
                        let color_char = match piece.color {
                            Color::White => 'w',
                            Color::Black => 'b',
                        };
                        row.push(format!(r#"{{"type":"{type_char}","color":"{color_char}"}}"#));
                    }
                    None => row.push("null".to_string()),
                }
            }
            board_json.push(format!("[{}]", row.join(",")));
        }
        
        format!("[{}]", board_json.join(","))
    }

    // ── Move queries ──────────────────────────────────────────────────────────

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

    pub fn legal_moves_san(&self) -> String {
        let moves = self.position.legal_moves();
        let sans: Vec<String> = moves
            .iter()
            .map(|m| format!("\"{}\"", san_with_suffix(&self.position, m)))
            .collect();
        format!("[{}]", sans.join(","))
    }

    // ── Making moves ─────────────────────────────────────────────────────────

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
                if let Ok(new_pos) = self.position.clone().play(m) {
                    self.position = new_pos;
                    self.san_history.push(san.clone());
                    format!(r#"{{"from":"{from_str}","to":"{to_str}","san":"{san}"}}"#)
                } else {
                    String::new()
                }
            }
            None => {
                // If it was a promotion move but no promo piece was specified, try defaulting to Queen
                if promo.is_none() {
                    let queen_promo = Some(Role::Queen);
                    let found_queen = moves.iter().find(|m| {
                        move_from(m) == Some(from)
                            && move_to(m) == to
                            && m.promotion() == queen_promo
                    });
                    if let Some(m) = found_queen {
                        let san = san_with_suffix(&self.position, m);
                        let from_str = move_from(m).map(|s| s.to_string()).unwrap_or_default();
                        let to_str = move_to(m).to_string();
                        if let Ok(new_pos) = self.position.clone().play(m) {
                            self.position = new_pos;
                            self.san_history.push(san.clone());
                            return format!(r#"{{"from":"{from_str}","to":"{to_str}","san":"{san}"}}"#);
                        }
                    }
                }
                String::new()
            }
        }
    }

    pub fn make_san_move(&mut self, san_str: &str) -> String {
        let san: San = match san_str.trim().parse() {
            Ok(s) => s,
            Err(_) => return String::new(),
        };

        if let Ok(m) = san.to_move(&self.position) {
            let from_str = move_from(&m).map(|s| s.to_string()).unwrap_or_default();
            let to_str = move_to(&m).to_string();
            let san_actual = san_with_suffix(&self.position, &m);
            if let Ok(new_pos) = self.position.clone().play(&m) {
                self.position = new_pos;
                self.san_history.push(san_actual.clone());
                format!(r#"{{"from":"{from_str}","to":"{to_str}","san":"{san_actual}"}}"#)
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    }

    // ── AI / Search interface ─────────────────────────────────────────────────

    pub fn search_depth(&self) -> u32 {
        DEFAULT_SEARCH_DEPTH
    }

    pub fn best_move(&mut self, depth: u32) -> String {
        if self.position.is_game_over() {
            return String::new();
        }
        // Limit TT size to prevent OOM in WASM
        if self.tt.len() > MAX_TT_ENTRIES {
            self.tt.clear();
        }
        search::best_move_san(&self.position, depth, &mut self.tt)
    }

    pub fn make_best_move(&mut self) -> String {
        let san = self.best_move(DEFAULT_SEARCH_DEPTH);
        if san.is_empty() {
            return String::new();
        }
        self.make_san_move(&san)
    }

    // ── Query helpers ──────────────────────────────────────────────────────────

    pub fn get_san_history(&self) -> String {
        format!("[{}]", self.san_history.iter().map(|s| format!("\"{}\"", s)).collect::<Vec<_>>().join(","))
    }

    pub fn castling_rights(&self) -> String {
        let castles = self.position.castles();
        let mut out = String::new();
        if castles.has(Color::White, CastlingSide::KingSide) { out.push('K'); }
        if castles.has(Color::White, CastlingSide::QueenSide) { out.push('Q'); }
        if castles.has(Color::Black, CastlingSide::KingSide) { out.push('k'); }
        if castles.has(Color::Black, CastlingSide::QueenSide) { out.push('q'); }
        out
    }

    pub fn en_passant_target(&self) -> String {
        match self.position.maybe_ep_square() {
            Some(s) => s.to_string(),
            None => String::new(),
        }
    }
}
