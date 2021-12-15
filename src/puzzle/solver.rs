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
            let all_perms = piece.all_perms();
            let mut solutions = 0;
            (0..board.width).for_each (|x: i32| {
                (0..board.height).for_each (|y: i32| {
                    all_perms.for_each (|p: Piece| {
                        let new_board = board.place_piece(&p,x,y);
                        new_board.map(|b: Board| {
                            solutions += solver(b, other_pieces)
                        })
                    });
                });
            });
            solutions
        }
    }
}