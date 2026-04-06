# ♜ Advanced Grandmaster AI

A modern, browser-based chess game featuring a robust AI opponent powered by Web Workers and Advanced Minimax algorithms.

## 🎯 Features

- ⚡ **Web Worker Architecture** - Fast performance that never blocks or freezes the user interface.
- 🎮 **Full Chess Rules** - Castling, en passant, pawn promotion, and strict check/checkmate validation (powered by `chess.js`).
- 🧠 **Alpha-Beta Pruning Minimax** - Optimizing search depths significantly allowing the AI to look several moves ahead.
- ♟️ **Positional Piece-Square Tables** - Detailed positional evaluation forcing the AI to control the center and protect the King.
- 🎨 **Premium Glassmorphism UI** - Modern responsive design with smooth animations.

## 📦 Files Structure

```
- index.html    # Main HTML game page, UI controller
- engine.js     # Background Web Worker (AI Logic)
- README.md     # This documentation
```

## 🚀 How to Run

### Option 1: Open Directly
*NOTE: Since this uses Web Workers, opening directly via `file://` might be blocked by browser CORS policies for workers.*

### Option 2: Local Server (Recommended)

```bash
# Using Python
python3 -m http.server

# Or Node.js
npx serve ./
```

Then navigate to `http://localhost:8000/index.html`

## 🎮 Game Controls

- Click a piece to select it
- Valid moves are highlighted with circle markers
- Captures are highlighted with a ring outline
- Match history logs automatically during the game

## 🔧 Architecture Stack

- HTML5/CSS3 Grid Layouts
- JavaScript (ES6+) for UI handling
- `chess.js` (via CDN) for game state and pseudo-legal move validation
- Web Worker API (`engine.js`) for background processing
- Alpha-Beta Pruning AI Search

## 🔗 Credits
Chess Rules Engine: [chess.js](https://github.com/jhlywa/chess.js)

---

Enjoy playing! ♟️