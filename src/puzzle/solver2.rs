use crate::puzzle::piece::{Piece};
use crate::puzzle::board::{Board};

pub fn solver2(board: Board, pieces: &[Piece], print: bool, first_layer: bool) -> usize {
    match pieces.len() {
        0 => {
            if print {
                println!("Placed all pieces");
                board.print();
            }
            1
        }
        _ => {
            let mut solutions = 0;
            match board.first_empty_space() {
                Some((x,y)) => {
                    for (p_ind, p) in pieces.iter().enumerate() {
                        for perm in &p.all_perms {
                            if x >= perm.offset {
                                board.place_piece(&perm, x - perm.offset, y).map(|b: Board| {
                                    let other_pieces = [&pieces[..p_ind], &pieces[p_ind + 1..]].concat();
                                    solutions += solver2(b, &other_pieces[..], print, false);
                                    if first_layer {
                                        println!("Solutions so far: {}", solutions);
                                    }
                                });
                            }
                        }
                    }
                },
                None => println!("Didn't find empty space even though there are pieces left")
            }
            solutions
        }
    }
}