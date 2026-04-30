# Chess Engine Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.8.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2026-04-30

### Added
- **UI Consistency**: Synchronized root UI and viewer UI with the new evaluation bar and polished design.
- **Version Bump**: Official release of the optimized WASM engine.

## [0.2.0] - 2026-04-29
- **Transposition Table**: Implemented Zobrist hashing to cache and reuse evaluations, significantly speeding up deep searches.
- **MVV-LVA Move Ordering**: Implemented "Most Valuable Victim - Least Valuable Aggressor" heuristic for more efficient Alpha-Beta pruning.
- **Mate Distance Scoring**: AI now prefers the shortest path to checkmate.
- **Visual Evaluation Bar**: Added a real-time board balance indicator to the UI.
- **SAN Suffixes**: Integrated `SanPlus` for proper `+` and `#` suffixes in move history.

### Fixed
- **Horizon Effect**: Refined quiescence search to stabilize tactical exchanges.
- **Game-Over Protection**: Added robust early-exit checks to prevent redundant search in terminal positions.
- **Promotion Handling**: Defaulted to Queen for promotion moves if no piece is specified by the UI.
- **King Safety**: Integrated positional penalties for exposed kings in the evaluation function.
- **Timer Sync**: Improved clock synchronization between moves.

### Refactored
- Cleaned up WASM interface to expose board evaluation.
- Optimized move generation and validation paths.
- Consolidated documentation for production readiness.

## [0.1.0] - Initial Release

### Added

#### Core Engine
- Full chess move generation (castling, en passant, promotions)
- Board state management (FEN loading, turn tracking)
- Game-over detection (checkmate, stalemate, insufficient material)
- Basic alpha-beta minimax AI

#### WASM Interface
- WebAssembly compilation for browser use
- Exported `ChessEngine` class to JavaScript
- FEN-based board representation
- SAN notation move history

#### Evaluation
- Material-based piece values
- Piece-square tables for positional play
- Basic pawn development bonuses

### Fixed

- Initial compilation issues
- WASM export configuration
- Basic move validation

### Dependencies
- shakmaty 0.27
- wasm-bindgen 0.2
- serde_json 1

[Unreleased]

### Planned

#### Performance
- Iterative deepening for progressive search
- Time management for real-time play
- Move ordering heuristics (history table, killer moves)
- Transposition table (optional)

#### Features
- NNUE (neural network) integration
- UCI engine protocol support
- PV (principal variation) output
- Threaded search for multi-core

#### Testing
- Unit tests for move generation
- Integration tests for game flow
- Performance benchmarks
- Mutation testing

#### Documentation
- API reference
- Performance benchmarks
- Contributing guide
- Performance tuning guide