use crate::puzzle::piece::{Piece};
use crate::puzzle::board::{Board};

pub fn solver(board: Board, pieces: &[Piece], print: bool) -> i32 {
    match pieces.split_first() {
        None => {
            if print {
                println!("Placed all pieces");
                board.print();
            }
            1
        }
        Some((piece, other_pieces)) => {
            if !board.find_available_space() {
                return 0;
            }
            let all_perms: Vec<Piece> = piece.all_perms();
            let mut solutions = 0;
            for x in 0..board.width {
                for y in 0..board.height {
                    for p in &all_perms {
                        board.place_piece(p,x,y).map(|b: Board| {
                                solutions += solver(b, other_pieces, print)
                            });
                        ()
                    }
                }
            }
            solutions
        }
    }
}