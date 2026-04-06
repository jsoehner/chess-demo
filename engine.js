// engine.js
// A Web Worker implementation of a Chess Engine
// Uses Alpha-Beta Pruning and Piece-Square tables for positional evaluation.

// We import chess.js into the worker to handle move generation
importScripts('https://cdnjs.cloudflare.com/ajax/libs/chess.js/0.10.3/chess.min.js');

const game = new Chess();

// Material values (approximate)
const pieceValues = {
    'p': 100,
    'n': 320,
    'b': 330,
    'r': 500,
    'q': 900,
    'k': 20000
};

// Piece-Square tables to encourage the AI to develop pieces centrally and protect the king.
// Indexed by board squares (0-63). For simplicity, arrays map A8->H8, A7->H7, etc.
const pawnEvalWhite = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [50, 50, 50, 50, 50, 50, 50, 50],
    [10, 10, 20, 30, 30, 20, 10, 10],
    [5,  5, 10, 25, 25, 10,  5,  5],
    [0,  0,  0, 20, 20,  0,  0,  0],
    [5, -5,-10,  0,  0,-10, -5,  5],
    [5, 10, 10,-20,-20, 10, 10,  5],
    [0,  0,  0,  0,  0,  0,  0,  0]
];

const knightEval = [
    [-50,-40,-30,-30,-30,-30,-40,-50],
    [-40,-20,  0,  0,  0,  0,-20,-40],
    [-30,  0, 10, 15, 15, 10,  0,-30],
    [-30,  5, 15, 20, 20, 15,  5,-30],
    [-30,  0, 15, 20, 20, 15,  0,-30],
    [-30,  5, 10, 15, 15, 10,  5,-30],
    [-40,-20,  0,  5,  5,  0,-20,-40],
    [-50,-40,-30,-30,-30,-30,-40,-50]
];

const bishopEvalWhite = [
    [-20,-10,-10,-10,-10,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5, 10, 10,  5,  0,-10],
    [-10,  5,  5, 10, 10,  5,  5,-10],
    [-10,  0, 10, 10, 10, 10,  0,-10],
    [-10, 10, 10, 10, 10, 10, 10,-10],
    [-10,  5,  0,  0,  0,  0,  5,-10],
    [-20,-10,-10,-10,-10,-10,-10,-20]
];

const rookEvalWhite = [
    [0,  0,  0,  0,  0,  0,  0,  0],
    [5, 10, 10, 10, 10, 10, 10,  5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [-5,  0,  0,  0,  0,  0,  0, -5],
    [0,  0,  0,  5,  5,  0,  0,  0]
];

const evalQuen = [
    [-20,-10,-10, -5, -5,-10,-10,-20],
    [-10,  0,  0,  0,  0,  0,  0,-10],
    [-10,  0,  5,  5,  5,  5,  0,-10],
    [ -5,  0,  5,  5,  5,  5,  0, -5],
    [  0,  0,  5,  5,  5,  5,  0, -5],
    [-10,  5,  5,  5,  5,  5,  0,-10],
    [-10,  0,  5,  0,  0,  0,  0,-10],
    [-20,-10,-10, -5, -5,-10,-10,-20]
];

const kingEvalWhite = [
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-30,-40,-40,-50,-50,-40,-40,-30],
    [-20,-30,-30,-40,-40,-30,-30,-20],
    [-10,-20,-20,-20,-20,-20,-20,-10],
    [ 20, 20,  0,  0,  0,  0, 20, 20],
    [ 20, 30, 10,  0,  0, 10, 30, 20]
];

function reverseArray(array) {
    return array.slice().reverse();
}

const pawnEvalBlack = reverseArray(pawnEvalWhite);
const bishopEvalBlack = reverseArray(bishopEvalWhite);
const rookEvalBlack = reverseArray(rookEvalWhite);
const kingEvalBlack = reverseArray(kingEvalWhite);

function evaluateBoard(gameInst) {
    let totalEvaluation = 0;
    const board = gameInst.board();

    for (let r = 0; r < 8; r++) {
        for (let c = 0; c < 8; c++) {
            const piece = board[r][c];
            if (piece) {
                totalEvaluation += getPieceValue(piece, r, c);
            }
        }
    }
    return totalEvaluation;
}

function getPieceValue(piece, r, c) {
    const isWhite = piece.color === 'w';
    let val = pieceValues[piece.type];
    
    // Add positional value
    switch(piece.type) {
        case 'p': val += isWhite ? pawnEvalWhite[r][c] : pawnEvalBlack[r][c]; break;
        case 'n': val += knightEval[r][c]; break;
        case 'b': val += isWhite ? bishopEvalWhite[r][c] : bishopEvalBlack[r][c]; break;
        case 'r': val += isWhite ? rookEvalWhite[r][c] : rookEvalBlack[r][c]; break;
        case 'q': val += evalQuen[r][c]; break;
        case 'k': val += isWhite ? kingEvalWhite[r][c] : kingEvalBlack[r][c]; break;
    }

    return isWhite ? val : -val;
}

function minimax(depth, gameInst, alpha, beta, isMaximizingPlayer) {
    if (depth === 0 || gameInst.game_over()) {
        return evaluateBoard(gameInst);
    }

    const moves = gameInst.moves();

    if (isMaximizingPlayer) {
        let bestVal = -Infinity;
        for (let i = 0; i < moves.length; i++) {
            gameInst.move(moves[i]);
            let value = minimax(depth - 1, gameInst, alpha, beta, !isMaximizingPlayer);
            gameInst.undo();
            bestVal = Math.max(bestVal, value);
            alpha = Math.max(alpha, bestVal);
            if (beta <= alpha) {
                break; // Beta cut-off
            }
        }
        return bestVal;
    } else {
        let bestVal = Infinity;
        for (let i = 0; i < moves.length; i++) {
            gameInst.move(moves[i]);
            let value = minimax(depth - 1, gameInst, alpha, beta, !isMaximizingPlayer);
            gameInst.undo();
            bestVal = Math.min(bestVal, value);
            beta = Math.min(beta, bestVal);
            if (beta <= alpha) {
                break; // Alpha cut-off
            }
        }
        return bestVal;
    }
}

self.onmessage = function(e) {
    const fen = e.data.fen;
    const depth = e.data.depth || 3;
    const aiColor = e.data.aiColor || 'b';

    game.load(fen);
    
    const moves = game.moves();
    if (moves.length === 0) {
        self.postMessage({ bestMove: null });
        return;
    }

    let bestMove = null;
    let bestValue = aiColor === 'w' ? -Infinity : Infinity;
    
    const isMaximizing = aiColor === 'w';

    // Sort moves to improve alpha-beta pruning efficiency 
    // (captures normally processed first -> faster cutoffs)
    moves.sort((a, b) => {
        const aCap = a.includes('x');
        const bCap = b.includes('x');
        if (aCap && !bCap) return -1;
        if (!aCap && bCap) return 1;
        return 0;
    });

    for (let i = 0; i < moves.length; i++) {
        const move = moves[i];
        game.move(move);
        
        let boardValue = minimax(depth - 1, game, -Infinity, Infinity, !isMaximizing);
        game.undo();

        if (isMaximizing) {
            if (boardValue > bestValue) {
                bestValue = boardValue;
                bestMove = move;
            }
        } else {
            if (boardValue < bestValue) {
                bestValue = boardValue;
                bestMove = move;
            }
        }
    }

    self.postMessage({ bestMove: bestMove });
};
