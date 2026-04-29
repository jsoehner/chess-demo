# Chess Engine

A lightweight chess engine written in Rust and compiled to WebAssembly for browser use.

## Features

- Full legal move generation (castling, en passant, promotions)
- Alpha-beta minimax with quiescence search
- Piece-square tables for positional play
- Check/checkmate detection
- Game-over handling (stalemate, insufficient material)
- WASM export for browser integration
- SAN notation move history
- Capture-first move ordering for improved AI
- King safety considerations in evaluation

## Installation

```bash
cd /mnt/c/Users/s5282032/chess-demo
npm install
```

## Usage

```javascript
const ChessEngine = await import('./chess-engine/chess_engine.js');
const engine = new ChessEngine();

// Load starting position
engine.loadFen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

// Get AI move
const bestMove = engine.makeBestMove();
console.log(bestMove); // e.g. "e4"

// Get legal moves
const moves = engine.legalMovesSan();
console.log(moves); // e.g. "[\"e4\", \"Nf3\", \"c3\", ...]"

// Make a move
engine.makeMove("e2", "e4", "n");
```

## API Reference

### Methods

- `constructor()` - Create new engine
- `loadFen(fen: string): boolean` - Load position from FEN
- `getFen(): string` - Get current FEN
- `getTurn(): string` - Get current turn ("w" or "b")
- `gameOver(): boolean` - Check if game is over
- `inCheck(): boolean` - Check if current player is in check
- `inCheckmate(): boolean` - Check if checkmate
- `inStalemate(): boolean` - Check if stalemate
- `inDraw(): boolean` - Check if draw
- `moveCount(): number` - Number of moves played
- `legalMovesFor(square: string): string` - Get legal moves from a square
- `legalMovesSan(): string` - Get all legal moves as SAN array
- `makeMove(from: string, to: string, promotion?: string): string` - Make move
- `makeSanMove(san: string): string` - Make move from SAN
- `makeBestMove(): string` - Get best move without depth
- `bestMove(depth: number): string` - Get best move with depth
- `searchDepth(): number` - Get default search depth

### Properties

- `sanHistory: string[]` - Array of SAN move history

## Build Commands

```bash
# Build for development
cargo build --target wasm32-unknown-unknown

# Build for release (optimized)
cargo build --release --target wasm32-unknown-unknown

# Run tests
cargo test

# Open in browser
cd /mnt/c/Users/s5282032/chess-demo && serve public
```

## Performance

- Depth 1: ~1-2 plies (instant)
- Depth 2: ~3-5 plies
- Depth 3: ~6-9 plies
- Depth 4: ~10-13 plies (default)

## License

MIT License
