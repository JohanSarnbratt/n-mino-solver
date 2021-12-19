use std::time::{Instant};
use chrono::NaiveDate;

use rust2::puzzle;
use crate::puzzle::piece::{Piece};

fn main() {

    //puzzle::piece::test_pieces();
    puzzle::board::example_date_board();
    //TODO benchmark available_space
    //_small_puzzle();
    //_small_puzzle2();
    //_tiny_puzzle2();
    //_date_puzzle();
}

fn _original_puzzle() {
    let vec_pieces = puzzle::piece::pieces_for_original();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    let board = puzzle::board::board8();
    let t1 = Instant::now();
    let solutions = puzzle::solver::solver(board, puzzle_pieces, true);
    println!("Found {} solutions in {}ms", solutions, t1.elapsed().as_millis())
}
fn _original_puzzle2() {
    //Found 12724 solutions in 354 260 ms
    let vec_pieces = puzzle::piece::pieces_for_original();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    let board = puzzle::board::board8();
    let t1 = Instant::now();
    let solutions = puzzle::solver2::solver2(board, puzzle_pieces, false, true);
    println!("Found {} solutions in {}ms", solutions, t1.elapsed().as_millis())
}
fn _small_puzzle() {
    //Found 344 solutions in 3472ms. optimised: true
    let vec_pieces = puzzle::piece::pieces_for_small();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    let board = puzzle::board::board6();
    let t1 = Instant::now();
    let solutions = puzzle::solver::solver(board, puzzle_pieces, false);
    println!("Found {} solutions in {}ms.", solutions, t1.elapsed().as_millis())
}
fn _small_puzzle2() {
    //Found 344 solutions in 3472ms. optimised: true
    let vec_pieces = puzzle::piece::pieces_for_small();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    let board = puzzle::board::board6();
    let t1 = Instant::now();
    let solutions = puzzle::solver2::solver2(board, puzzle_pieces, false, true);
    println!("Found {} solutions in {}ms.", solutions, t1.elapsed().as_millis())
}
fn _tiny_puzzle2() {
    //Found 344 solutions in 3472ms. optimised: true
    let vec_pieces = puzzle::piece::pieces_board_4();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    let board = puzzle::board::board4();
    let t1 = Instant::now();
    let solutions = puzzle::solver2::solver2(board, puzzle_pieces, true, true);
    println!("Found {} solutions in {}ms.", solutions, t1.elapsed().as_millis())
}
fn _date_puzzle() {
    let vec_pieces = puzzle::piece::pieces_for_date();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    let board = puzzle::board::date_board(NaiveDate::from_yo(2021, 300));
    let t1 = Instant::now();
    let solutions = puzzle::solver2::solver2(board, puzzle_pieces, false, true);
    println!("Found {} solutions in {}ms", solutions, t1.elapsed().as_millis())
}