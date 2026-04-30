# ♜ Advanced Grandmaster AI

A modern, high-performance browser-based chess game featuring a robust AI opponent built with Rust and WebAssembly.

---

## 🎯 Key Features

- 🧠 **WASM-Native Engine** — Optimized Alpha-Beta search compiled from Rust for near-native performance in the browser.
- 📊 **Real-time Evaluation Bar** — Visual feedback of the engine's assessment of the board.
- ⚡ **Self-Contained** — No CDN dependencies; fully functional offline once loaded.
- 🎮 **Full Chess Rules** — Castling, en passant, pawn promotion, and check/checkmate detection via the [shakmaty](https://github.com/niklasf/shakmaty) crate.
- 🧠 **Search Optimizations** — Transposition Tables (Zobrist), MVV-LVA move ordering, and quiescence search for tactical stability.
- 🌐 **Frontier LLM Backend** — Optional move selection via Gemini, Claude, or local-OpenAI models.
- 🎨 **Premium Glassmorphism UI** — Stunning modern design with smooth animations and high-quality SVG pieces.

---

## 📦 Project Structure

```
chess-demo/
├── index.html          # Main viewer page (WASM-powered)
├── pkg/                # Generated WASM artifacts (JS glue + binary)
├── chess-engine/       # Rust source code for the engine
│   ├── src/
│   │   ├── lib.rs      # WASM-exported API
│   │   ├── eval.rs     # Material + positional evaluation
│   │   └── search.rs   # Alpha-beta search logic
│   └── Cargo.toml      # Rust dependencies
├── build.sh            # Compile Rust → WASM
├── package.sh          # Create a distributable zip
└── README.md           # You are here
```

---

## 🚀 Getting Started

### Option A — Run the pre-built version
1. Ensure you have Python installed (or any local HTTP server).
2. Run `./build.sh` (if `pkg/` is missing).
3. Start a server: `python3 -m http.server 8000`.
4. Open **http://localhost:8000** in your browser.

### Option B — Build from source
**Prerequisites:**
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

**Build:**
```bash
./build.sh
```

---

## 🛠️ Technical Details

### Search & Evaluation
The engine uses a **Minimax** algorithm with **Alpha-Beta pruning**. To improve performance and stability, it includes:
- **Transposition Tables**: Caches positions to avoid redundant calculations.
- **Quiescence Search**: Prevents the "horizon effect" by searching capture sequences to stability.
- **Move Ordering**: Uses MVV-LVA to search the most promising moves first, increasing pruning efficiency.
- **Mate Distance Scoring**: Prioritizes the shortest path to checkmate.

### Backend Options
While the **WASM Core** is the default, you can also connect to frontier LLMs (Gemini, Claude, GPT) to see how modern language models play chess.

---

Enjoy playing! ♟️