// examples/chess/src/lib.rs
use rust2ts::{rust2ts, rust2ts_module};

#[rust2ts_module]
pub mod chess {
    // Структуры
    #[rust2ts]
    pub struct Board {
        squares: [[i32; 8]; 8],
        turn: i32,
    }
    
    #[rust2ts]
    pub struct Move {
        from_x: i32,
        from_y: i32,
        to_x: i32,
        to_y: i32,
    }
    
    // Константы
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
    
    // Функции
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
    
    #[rust2ts]
    pub fn make_move(board: Board, mv: Move) -> Board {
        let mut new_board = board;
        let piece = board.squares[mv.from_x as usize][mv.from_y as usize];
        new_board.squares[mv.from_x as usize][mv.from_y as usize] = 0;
        new_board.squares[mv.to_x as usize][mv.to_y as usize] = piece;
        new_board.turn = 1 - board.turn;
        new_board
    }
    
    #[rust2ts]
    pub fn initial_board() -> Board {
        Board {
            squares: [
                [-4, -2, -3, -5, -6, -3, -2, -4],
                [-1, -1, -1, -1, -1, -1, -1, -1],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0],
                [1, 1, 1, 1, 1, 1, 1, 1],
                [4, 2, 3, 5, 6, 3, 2, 4],
            ],
            turn: 0,
        }
    }
    
    // Вспомогательная функция - не экспортируется
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
