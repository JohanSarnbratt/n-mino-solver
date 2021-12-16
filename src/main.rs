use std::time::{Duration, Instant};

use rust2::puzzle;
use crate::puzzle::piece::{Piece};

fn main() {

    let vec_pieces = puzzle::piece::pieces_board_4();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    //puzzle::piece::test_pieces();
    //puzzle::board::test_board();
    let board = puzzle::board::board4();
    let t1 = Instant::now();
    let solutions = puzzle::solver::solver(board, puzzle_pieces, false);
    println!("Found {} solutions in {}ms", solutions, t1.elapsed().as_millis())
}
