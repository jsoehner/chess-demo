// search.rs – Alpha-beta minimax search, ported from engine.js

use shakmaty::{Chess, Color, Move, Position};

use crate::eval::evaluate;

// ── Public entry point ────────────────────────────────────────────────────────
/// Returns the best move for the current side in SAN notation, or an empty
/// string if there are no legal moves.
pub fn best_move_san(pos: &Chess, depth: u32) -> String {
    let moves = pos.legal_moves();
    if moves.is_empty() {
        return String::new();
    }

    let is_white = pos.turn() == Color::White;
    let mut best_val = if is_white { i32::MIN } else { i32::MAX };
    let mut best: Option<Move> = None;

    // Sort captures first (improves alpha-beta pruning efficiency)
    let mut sorted: Vec<Move> = moves.into_iter().collect();
    sorted.sort_by_key(|m| if m.is_capture() { 0 } else { 1 });

    for m in &sorted {
        if let Ok(child) = pos.clone().play(m) {
            let val = minimax(&child, depth.saturating_sub(1), i32::MIN + 1, i32::MAX - 1, !is_white);
            if is_white && val > best_val || !is_white && val < best_val {
                best_val = val;
                best = Some(m.clone());
            }
        }
    }

    match best {
        Some(ref m) => {
            use shakmaty::san::San;
            San::from_move(pos, m).to_string()
        }
        None => String::new(),
    }
}

// ── Recursive minimax ─────────────────────────────────────────────────────────
fn minimax(pos: &Chess, depth: u32, mut alpha: i32, mut beta: i32, maximizing: bool) -> i32 {
    if depth == 0 || pos.is_game_over() {
        return evaluate(pos);
    }

    let moves = pos.legal_moves();

    if maximizing {
        let mut best = i32::MIN + 1;
        for m in &moves {
            if let Ok(child) = pos.clone().play(m) {
                let val = minimax(&child, depth - 1, alpha, beta, false);
                if val > best {
                    best = val;
                }
                if best > alpha {
                    alpha = best;
                }
                if beta <= alpha {
                    break; // beta cut-off
                }
            }
        }
        best
    } else {
        let mut best = i32::MAX - 1;
        for m in &moves {
            if let Ok(child) = pos.clone().play(m) {
                let val = minimax(&child, depth - 1, alpha, beta, true);
                if val < best {
                    best = val;
                }
                if best < beta {
                    beta = best;
                }
                if beta <= alpha {
                    break; // alpha cut-off
                }
            }
        }
        best
    }
}
