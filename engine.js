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

// Quiescence search for captures only (avoids horizon effect)
function quiescence(gameInst, alpha, beta, isMaximizingPlayer) {
    const captures = gameInst.moves({ verbose: true }).filter(m => 
        m.flags.includes('c') || m.flags.includes('e') || m.flags.includes('p')
    );
    
    if (captures.length === 0 || gameInst.game_over()) {
        return evaluateBoard(gameInst);
    }
    
    // Sort captures by value (greedy best-first ordering)
    captures.sort((a, b) => {
        const valA = getPieceValue(gameInst.board()[a.to][0], a.to[0], a.to[1]);
        const valB = getPieceValue(gameInst.board()[b.to][0], b.to[0], b.to[1]);
        return valB - valA;
    });
    
    let bestVal = isMaximizingPlayer ? -Infinity : Infinity;
    
    for (let move of captures) {
        gameInst.move(move);
        let val = quiescence(gameInst, alpha, beta, !isMaximizingPlayer);
        gameInst.undo();
        
        if (isMaximizingPlayer && val > bestVal) {
            bestVal = val;
            alpha = bestVal;
        } else if (!isMaximizingPlayer && val < bestVal) {
            bestVal = val;
            beta = bestVal;
        }
        
        if (beta <= alpha) break; // Beta cutoff
    }
    
    return bestVal;
}

function minimax(depth, gameInst, alpha, beta, isMaximizingPlayer) {
    // Early exit for game-over positions
    if (gameInst.game_over() || depth === 0) {
        // Check for capture at depth 0 - use quiescence
        const lastMove = gameInst.history({ verbose: true }).pop();
        if (lastMove && (lastMove.flags.includes('c') || lastMove.flags.includes('e') || lastMove.flags.includes('p'))) {
            return quiescence(gameInst, alpha, beta, isMaximizingPlayer);
        }
        return evaluateBoard(gameInst);
    }

    const moves = gameInst.moves();
    
    // Early exit: no legal moves = game over
    if (moves.length === 0) {
        if (gameInst.in_checkmate()) {
            return isMaximizingPlayer ? 100000 : -100000;
        }
        // Stalemate or draw
        return 0;
    }

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
    
    const verboseMoves = game.moves({ verbose: true });
    if (verboseMoves.length === 0) {
        self.postMessage({ bestMove: null, error: 'Game over' });
        return;
    }

    let bestMove = null;
    let bestValue = aiColor === 'w' ? -Infinity : Infinity;
    let bestCount = 0;
    
    const isMaximizing = aiColor === 'w';

    // Sort moves with improved ordering
    // Order: checks > captures > promotions > knight moves > other captures > quiet moves
    moves.sort((a, b) => {
        const aChecks = a.flags.includes('C') || a.flags.includes('+');
        const bChecks = b.flags.includes('C') || b.flags.includes('+');
        if (aChecks && !bChecks) return -1;
        if (!aChecks && bChecks) return 1;
        
        const aCaptures = a.includes('x') && !a.includes('=');
        const bCaptures = b.includes('x') && !b.includes('=');
        if (aCaptures && !bCaptures) return -1;
        if (!aCaptures && bCaptures) return 1;
        
        const aPromos = a.includes('=');
        const bPromos = b.includes('=');
        if (aPromos && !bPromos) return -1;
        if (!aPromos && bPromos) return 1;
        
        const aKnights = a.includes('N');
        const bKnights = b.includes('N');
        if (aKnights && !bKnights) return -1;
        if (!aKnights && bKnights) return 1;
        
        return 0;
    });

    for (let i = 0; i < moves.length; i++) {
        const move = moves[i];
        game.move(move);
        
        // Use quiescence search for captures
        let boardValue;
        if (move.flags.includes('c') || move.flags.includes('e') || move.flags.includes('p')) {
            boardValue = quiescence(game, -Infinity, Infinity, !isMaximizing);
        } else {
            boardValue = minimax(depth - 1, game, -Infinity, Infinity, !isMaximizing);
        }
        game.undo();

        if (isMaximizing) {
            if (boardValue > bestValue) {
                bestValue = boardValue;
                bestMove = move;
                bestCount = 1;
            } else if (boardValue === bestValue) {
                bestCount++;
            }
        } else {
            if (boardValue < bestValue) {
                bestValue = boardValue;
                bestMove = move;
                bestCount = 1;
            } else if (boardValue === bestValue) {
                bestCount++;
            }
        }
    }

    self.postMessage({ bestMove: bestMove || verboseMoves[0], error: bestMove ? null : 'Error: No moves found' });
};
