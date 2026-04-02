"# ♟️ Chess WASM Demo

A modern, browser-based chess game powered by **WebAssembly** (WASM) for optimal performance.

## 🎯 Features

- ⚡ **WebAssembly Game Engine** - Fast, native-code performance
- 🎮 Full chess rules (castling, en passant, promotion)
- 🎨 Clean, responsive UI with intuitive controls
- 🏆 Check/checkmate detection and game over handling
- 🔄 Fallback to browser JS if WASM unavailable

## 📦 Files Structure

```
src/
-chess-wasm-demo.html  # Main HTML game page
src/chess-wasm-engine.js      # Chess game logic (WASM/JS)
README.md                      # This documentation
```

## 🚀 How to Run

### Option 1: Open Directly

1. Copy `src/chess-wasm-demo.html` to your browser
2. Open it in Chrome, Firefox, Safari, or Edge
3. Enjoy the game!

### Option 2: Local Server

```bash
# Using Python
python3 -m http.server

# Or Node.js
npx serve ./
```

Then navigate to `http://localhost:8000/src/chess-wasm-demo.html`

## 🎮 Game Controls

- Click a piece to select it
- Valid moves are highlighted in light green
- Click a highlighted square to move
- Game automatically detects check/checkmate

## 🔧 WASM vs Browser JS

The demo tries to load a WASM chess engine first:
- WASM version: Faster, native performance
- Browser JS fallback: Chess.js library (if WASM unavailable)

## 📝 License

MIT License - Feel free to modify and distribute!

## 🎨 Screenshots

```
┌────────────────────────────────────┐
│  ♟️  Chess WASM Demo               │
├────────────────────────────────────┤
│                                    │
│  [8x8 Chess Board]                 │
│                                    │
│  WASM Engine: ✓ Loaded ✓         │
│                                    │
└────────────────────────────────────┘
```

## 🛠️ Tech Stack

- HTML5/CSS3
- JavaScript (ES6+)
- WebAssembly (WASM)
- Unicode chess pieces
- Zero external dependencies

## 🔗 Credits

Chess pieces: Unicode Chess Symbols  
Game Logic: Custom WASM implementation  
UI Design: Modern, responsive layout

---

Enjoy playing chess with WASM! ♟️
"