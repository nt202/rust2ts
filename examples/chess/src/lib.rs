// examples/chess/src/lib.rs
use rust2ts::{rust2ts, rust2ts_module};

#[rust2ts_module]
pub mod chess {
    #[rust2ts]
    #[derive(Clone, Copy)]
    pub struct Board {
        // 0 = пусто, 1-6 = белые фигуры, -1..-6 = черные фигуры
        squares: [[i32; 8]; 8],
        turn: i32, // 0 = белые, 1 = черные
    }
    
    #[rust2ts]
    pub const PIECE_PAWN: i32 = 1;
    #[rust2ts]
    pub const PIECE_KNIGHT: i32 = 2;
    #[rust2ts]
    pub const PIECE_BISHOP: i32 = 3;
    #[rust2ts]
    pub const PIECE_ROOK: i32 = 4;
    #[rust2ts]
    pub const PIECE_QUEEN: i32 = 5;
    #[rust2ts]
    pub const PIECE_KING: i32 = 6;
    
    #[rust2ts]
    pub fn evaluate_material(board: Board) -> i32 {
        let mut score = 0;
        for i in 0..8 {
            for j in 0..8 {
                let piece = board.squares[i][j];
                if piece != 0 {
                    score += piece_value(piece);
                }
            }
        }
        score
    }
    
    // Эта функция не экспортируется (нет #[rust2ts])
    fn piece_value(piece: i32) -> i32 {
        match piece.abs() {
            PIECE_PAWN => 1,
            PIECE_KNIGHT => 3,
            PIECE_BISHOP => 3,
            PIECE_ROOK => 5,
            PIECE_QUEEN => 9,
            PIECE_KING => 0,
            _ => 0,
        }
    }
}
