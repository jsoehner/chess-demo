use shakmaty::{Chess, Color, Move, Position, Role};
use shakmaty::zobrist::ZobristHash;
use crate::eval;
use crate::eval::evaluate;
use std::collections::HashMap;

const CHECKMATE_VAL: i32 = 100_000;

/// Returns the SAN string of the best move found.
pub fn best_move_san(pos: &Chess, depth: u32, tt: &mut HashMap<u64, (i32, u32)>) -> String {
    let moves = pos.legal_moves();
    if moves.is_empty() {
        return String::new();
    }

    let mut best_moves: Vec<Move> = Vec::new();
    let is_white = pos.turn() == Color::White;
    let mut best_val = if is_white { i32::MIN } else { i32::MAX };

    let mut alpha = i32::MIN;
    let mut beta = i32::MAX;

    // Move ordering
    let mut sorted_moves: Vec<Move> = moves.into_iter().collect();
    sort_moves_internal(pos, &mut sorted_moves);

    for m in sorted_moves {
        if let Ok(child) = pos.clone().play(&m) {
            let val = minimax(&child, depth - 1, alpha, beta, !is_white, 1, tt);
            
            if is_white {
                if val > best_val {
                    best_val = val;
                    best_moves.clear();
                    best_moves.push(m);
                } else if val == best_val {
                    best_moves.push(m);
                }
                alpha = alpha.max(best_val);
            } else {
                if val < best_val {
                    best_val = val;
                    best_moves.clear();
                    best_moves.push(m);
                } else if val == best_val {
                    best_moves.push(m);
                }
                beta = beta.min(best_val);
            }
        }
    }

    if let Some(m) = best_moves.first() {
        shakmaty::san::SanPlus::from_move(pos.clone(), m).to_string()
    } else {
        String::new()
    }
}

fn sort_moves_internal(pos: &Chess, moves: &mut [Move]) {
    moves.sort_by_cached_key(|m| {
        let mut score = 0;
        if let Some(capture) = m.capture() {
            // MVV-LVA: Most Valuable Victim - Least Valuable Aggressor
            let victim_val = eval::piece_value(capture);
            let attacker_role = pos.board().piece_at(m.from().unwrap()).map(|p| p.role).unwrap_or(Role::Pawn);
            let attacker_val = eval::piece_value(attacker_role);
            score += 1000 + (victim_val - attacker_val / 10);
        }
        if m.is_promotion() {
            score += 900;
        }
        -score // higher score first
    });
}

fn minimax(
    pos: &Chess, 
    depth: u32, 
    mut alpha: i32, 
    mut beta: i32, 
    maximizing: bool, 
    depth_from_root: i32,
    tt: &mut HashMap<u64, (i32, u32)>
) -> i32 {
    let hash = pos.zobrist_hash::<shakmaty::zobrist::Zobrist64>(shakmaty::EnPassantMode::Legal).0;
    if let Some(&(val, d)) = tt.get(&hash) {
        if d >= depth {
            return val;
        }
    }

    if pos.is_game_over() {
        if pos.is_checkmate() {
            // Prefer shorter mates: subtract depth_from_root
            return if maximizing { -CHECKMATE_VAL + depth_from_root } else { CHECKMATE_VAL - depth_from_root };
        }
        return evaluate(pos);
    }

    if depth == 0 {
        return quiescence(pos, alpha, beta, maximizing, 0);
    }

    let mut moves: Vec<Move> = pos.legal_moves().into_iter().collect();
    sort_moves_internal(pos, &mut moves);

    if maximizing {
        let mut best = i32::MIN;
        for m in moves {
            if let Ok(child) = pos.clone().play(&m) {
                let val = minimax(&child, depth - 1, alpha, beta, false, depth_from_root + 1, tt);
                best = best.max(val);
                alpha = alpha.max(best);
                if beta <= alpha {
                    break;
                }
            }
        }
        tt.insert(hash, (best, depth));
        best
    } else {
        let mut best = i32::MAX;
        for m in moves {
            if let Ok(child) = pos.clone().play(&m) {
                let val = minimax(&child, depth - 1, alpha, beta, true, depth_from_root + 1, tt);
                best = best.min(val);
                beta = beta.min(best);
                if beta <= alpha {
                    break;
                }
            }
        }
        tt.insert(hash, (best, depth));
        best
    }
}

fn quiescence(pos: &Chess, mut alpha: i32, mut beta: i32, maximizing: bool, q_depth: u32) -> i32 {
    let standby = evaluate(pos);
    
    if maximizing {
        if standby >= beta { return beta; }
        alpha = alpha.max(standby);
    } else {
        if standby <= alpha { return alpha; }
        beta = beta.min(standby);
    }

    if q_depth >= 4 {
        return standby;
    }

    let moves = pos.legal_moves();
    let mut captures: Vec<Move> = moves.into_iter()
        .filter(|m| m.is_capture() || m.is_en_passant() || pos.is_check())
        .collect();

    if captures.is_empty() {
        return standby;
    }

    // Sort captures for better pruning
    sort_moves_internal(pos, &mut captures);

    if maximizing {
        let mut best = standby;
        for m in captures {
            if let Ok(child) = pos.clone().play(&m) {
                let val = quiescence(&child, alpha, beta, false, q_depth + 1);
                best = best.max(val);
                alpha = alpha.max(best);
                if beta <= alpha {
                    break;
                }
            }
        }
        best
    } else {
        let mut best = standby;
        for m in captures {
            if let Ok(child) = pos.clone().play(&m) {
                let val = quiescence(&child, alpha, beta, true, q_depth + 1);
                best = best.min(val);
                beta = beta.min(best);
                if beta <= alpha {
                    break;
                }
            }
        }
        best
    }
}
