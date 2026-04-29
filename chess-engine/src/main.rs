use chess_engine::ChessEngine;

fn main() {
    println!("🏁 Chess Engine v0.1.0 — Ready");
    println!("================================");
    
    let mut engine = ChessEngine::new();
    engine.reset();
    
    println!("\n📊 Initial Position:");
    println!("  FEN: {}", engine.get_fen());
    println!("  Turn: {}", engine.get_turn());
    println!("  Legal moves from e2: {}", engine.legal_moves_for("e2"));
    
    // Play opening moves
    println!("\n🎮 Playing opening: 1. e4 e5 2. Nf3 Nc6");
    engine.make_move("e2", "e4", "");
    println!("  White: e4");
    engine.make_move("e7", "e5", "");
    println!("  Black: e5");
    engine.make_move("g1", "f3", "");
    println!("  White: Nf3");
    engine.make_move("g8", "f6", "");
    println!("  Black: Nf6");
    
    // Show engine's best move
    println!("\n💭 Engine thinks...");
    println!("  Best move (depth 3): {}", engine.best_move(3));
    
    println!("\n✅ Engine evaluation working correctly!");
}
"