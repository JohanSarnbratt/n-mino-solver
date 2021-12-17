use std::time::{Instant};

use rust2::puzzle;
use crate::puzzle::piece::{Piece};

fn main() {

    //puzzle::piece::test_pieces();
    //puzzle::board::test_board();
    //TODO benchmark available_space
    _small_puzzle(false);
    _small_puzzle(true);
    _small_puzzle(false);
    _small_puzzle(true);
}

fn _original_puzzle() {
    let vec_pieces = puzzle::piece::pieces_for_original();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    let board = puzzle::board::board8();
    let t1 = Instant::now();
    let solutions = puzzle::solver::solver(board, puzzle_pieces, true, false);
    println!("Found {} solutions in {}ms", solutions, t1.elapsed().as_millis())
}
fn _small_puzzle(opt: bool) {
    let vec_pieces = puzzle::piece::pieces_for_small();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    let board = puzzle::board::board6();
    let t1 = Instant::now();
    let solutions = puzzle::solver::solver(board, puzzle_pieces, false, true);
    println!("Found {} solutions in {}ms. optimised: {}", solutions, t1.elapsed().as_millis(), opt)
}
fn _date_puzzle() {
    let vec_pieces = puzzle::piece::pieces_for_date();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    let board = puzzle::board::date_board();
    let t1 = Instant::now();
    let solutions = puzzle::solver::solver(board, puzzle_pieces, true, false);
    println!("Found {} solutions in {}ms", solutions, t1.elapsed().as_millis())
}