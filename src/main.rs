
use rust2::puzzle;
use crate::puzzle::piece::{Piece};

fn main() {

    let vec_pieces = puzzle::piece::pieces();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    //puzzle::piece::test_pieces();
    //puzzle::board::test_board();
    let board = puzzle::board::board4();
    let solutions = puzzle::solver::solver(board, puzzle_pieces);
    println!("Found {} solutions", solutions)
}
