use crate::puzzle::piece::{Piece};
use crate::puzzle::board::{Board};

pub fn solver(board: Board, pieces: &[Piece]) -> i32 {
    match pieces.split_first() {
        None => {
            println!("Placed all pieces");
            board.print();
            1
        }
        Some((piece, other_pieces)) => {
            let all_perms: [Piece; 8] = piece.all_perms();
            let mut solutions = 0;
            for x in 0..board.width {
                for y in 0..board.height {
                    all_perms.iter().for_each (|p: &Piece| {
                        board.place_piece(p,x,y).map(|b: Board| {
                                solutions += solver(b, other_pieces)
                            });
                        ()
                    });
                    ()
                }
            }
            solutions
        }
    }
}