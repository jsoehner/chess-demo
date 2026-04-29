// eval.rs – Piece-square table evaluation, ported from engine.js

use shakmaty::{Board, Color, Position, Role, Square};

// ── Material values ───────────────────────────────────────────────────────────
pub fn piece_value(role: Role) -> i32 {
    match role {
        Role::Pawn => 100,
        Role::Knight => 320,
        Role::Bishop => 330,
        Role::Rook => 500,
        Role::Queen => 900,
        Role::King => 20_000,
    }
}

// ── King safety values (penalize exposed kings) ────────────────────────────────
/// Negative = good for king (safe), Positive = bad for king (dangerous)
const KING_SAFE: [[i32; 8]; 8] = [
    // Rank 8 (row 0)
    [-20, -30, -30, -40, -40, -30, -30, -20],
    // Rank 7
    [-10, -20, -20, -25, -25, -20, -20, -10],
    // Rank 6
    [-5, -10, -10, -15, -15, -10, -10, -5],
    // Rank 5
    [0, -5, -5, -10, -10, -5, -5, 0],
    // Rank 4
    [0, -5, -5, -10, -10, -5, -5, 0],
    // Rank 3
    [-5, -5, -5, -5, -5, -5, -5, -5],
    // Rank 2
    [-5, -10, -10, -5, -5, -10, -10, -5],
    // Rank 1 (row 7) 
    [-20, -30, -30, -40, -40, -30, -30, -20],
];

// ── Piece-square tables (row 0 = rank 8, row 7 = rank 1) ─────────────────────
// For white  : table[7 - rank][file]
// For black  : table[rank][file]  (mirrored – same data, opposite indexing)

const PAWN_WHITE: [[i32; 8]; 8] = [
    [  0,  0,  0,  0,  0,  0,  0,  0],
    [ 50, 50, 50, 50, 50, 50, 50, 50],
    [ 10, 10, 20, 30, 30, 20, 10, 10],
    [  5,  5, 10, 25, 25, 10,  5,  5],
    [  0,  0,  0, 20, 20,  0,  0,  0],
    [  5, -5,-10,  0,  0,-10, -5,  5],
    [  5, 10, 10,-20,-20, 10, 10,  5],
    [  0,  0,  0,  0,  0,  0,  0,  0],
];

const KNIGHT: [[i32; 8]; 8] = [
    [-50,-40,-30,-30,-30,-30,-40,-50],
    [-40,-20,  0,  0,  0,  0,-20,-40],
    [-30,  0, 10, 15, 15, 10,  0,-30],
    [-30,  5, 15, 20, 20, 15,  5,-30],
    [-30,  0, 15, 20, 20, 15,  0,-30],
    [-30,  5, 10, 15, 15, 10,  5,-30],
    [-40,-20,  0,  5,  5,  0,-20,-40],
    [-50,-40,-30,-30,-30,-30,-40,-50],
];

const BISHOP_WHITE: [[i32; 8]; 8] = [
    [-20,-10,-10,-10,-10,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5, 10, 10,  5,  0,-10],
    [-10,  5,  5, 10, 10,  5,  5,-10],
    [-10,  0, 10, 10, 10, 10,  0,-10],
    [-10, 10, 10, 10, 10, 10, 10,-10],
    [-10,  5,  0,  0,  0,  0,  5,-10],
    [-20,-10,-10,-10,-10,-10,-10,-20],
];

const ROOK_WHITE: [[i32; 8]; 8] = [
    [  0,  0,  0,  0,  0,  0,  0,  0],
    [  5, 10, 10, 10, 10, 10, 10,  5],
    [ -5,  0,  0,  0,  0,  0,  0, -5],
    [ -5,  0,  0,  0,  0,  0,  0, -5],
    [ -5,  0,  0,  0,  0,  0,  0, -5],
    [ -5,  0,  0,  0,  0,  0,  0, -5],
    [ -5,  0,  0,  0,  0,  0,  0, -5],
    [  0,  0,  0,  5,  5,  0,  0,  0],
];

const QUEEN: [[i32; 8]; 8] = [
    [-20,-10,-10, -5, -5,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5,  5,  5,  5,  0,-10],
    [ -5,  0,  5,  5,  5,  5,  0, -5],
    [  0,  0,  5,  5,  5,  5,  0, -5],
    [-10,  5,  5,  5,  5,  5,  0,-10],
    [-10,  0,  5,  0,  0,  0,  0,-10],
    [-20,-10,-10, -5, -5,-10,-10,-20],
];

const KING_WHITE: [[i32; 8]; 8] = [
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-20,-30,-30,-40,-40,-30,-30,-20],
    [-10,-20,-20,-20,-20,-20,-20,-10],
    [ 20, 20,  0,  0,  0,  0, 20, 20],
    [ 20, 30, 10,  0,  0, 10, 30, 20],
];

// ── Positional value ──────────────────────────────────────────────────────────
fn positional_value(role: Role, color: Color, sq: Square) -> i32 {
    let rank = sq.rank() as usize; // 0 = rank 1 (white back rank), 7 = rank 8
    let file = sq.file() as usize; // 0 = a-file, 7 = h-file
    // White: row 0 = rank 8 ⟹ row = 7 - rank
    // Black: mirrored ⟹ row = rank
    let (row_w, row_b) = (7 - rank, rank);
    match (role, color) {
        (Role::Pawn,   Color::White) => PAWN_WHITE[row_w][file],
        (Role::Pawn,   Color::Black) => PAWN_WHITE[row_b][file],
        (Role::Knight, _           ) => KNIGHT[row_w][file],
        (Role::Bishop, Color::White) => BISHOP_WHITE[row_w][file],
        (Role::Bishop, Color::Black) => BISHOP_WHITE[row_b][file],
        (Role::Rook,   Color::White) => ROOK_WHITE[row_w][file],
        (Role::Rook,   Color::Black) => ROOK_WHITE[row_b][file],
        (Role::Queen,  _           ) => QUEEN[row_w][file],
        (Role::King,   Color::White) => {
            // Combine positional table with king safety
            let base = KING_WHITE[row_w][file];
            let safety = KING_SAFE[row_w][file];
            base + safety
        },
        (Role::King,   Color::Black) => {
            // Combine positional table with king safety
            let base = KING_WHITE[row_b][file];
            let safety = KING_SAFE[row_b][file];
            base + safety
        },
    }
}

// ── Board evaluation (positive = good for white) ─────────────────────────────
pub fn evaluate<P: Position>(pos: &P) -> i32 {
    let board: &Board = pos.board();
    let mut total: i32 = 0;
    
    // Material evaluation
    for sq in Square::ALL {
        if let Some(piece) = board.piece_at(sq) {
            let val = piece_value(piece.role) + positional_value(piece.role, piece.color, sq);
            if piece.color == Color::White {
                total += val;
            } else {
                total -= val;
            }
        }
    }
    
    // Small bonus for white (slight advantage for first player at equal positions)
    total + 1
}
