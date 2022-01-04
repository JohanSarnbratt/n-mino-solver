use std::time::{Instant};
use chrono::NaiveDate;

use rust2::puzzle;
use crate::puzzle::piece::{Piece};

fn main() {

    //puzzle::piece::test_pieces();
    //puzzle::board::example_date_board();
    //TODO benchmark available_space
    //_small_puzzle();
    //_small_puzzle2();
    //_tiny_puzzle2();
    _date_puzzle();
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
    let solutions = puzzle::solver2::solver2(board, puzzle_pieces, false);
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
    let solutions = puzzle::solver2::solver2(board, puzzle_pieces, false);
    println!("Found {} solutions in {}ms.", solutions, t1.elapsed().as_millis())
}
fn _tiny_puzzle2() {
    //Found 344 solutions in 3472ms. optimised: true
    let vec_pieces = puzzle::piece::pieces_board_4();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    let board = puzzle::board::board4();
    let t1 = Instant::now();
    let solutions = puzzle::solver2::solver2(board, puzzle_pieces, true);
    println!("Found {} solutions in {}ms.", solutions, t1.elapsed().as_millis())
}
fn _date_puzzle() {
    let vec_pieces = puzzle::piece::pieces_for_date();
    let puzzle_pieces: &[Piece] = vec_pieces.as_slice();
    for day in 1..32 {
        let board = puzzle::board::date_board(NaiveDate::from_ymd(2021, 12, day));
        let t1 = Instant::now();
        let solutions = puzzle::solver2::solver2(board, puzzle_pieces, false);
        println!("Found {} solutions in {}ms", solutions, t1.elapsed().as_millis())
    }
}
/*
Creating board for 2021 12 1 Wed
Found 1245 solutions in 29722ms
Creating board for 2021 12 2 Thu
Found 1917 solutions in 30862ms
Creating board for 2021 12 3 Fri
Found 1191 solutions in 35558ms
Creating board for 2021 12 4 Sat
Found 1732 solutions in 41435ms
Creating board for 2021 12 5 Sun
Found 278 solutions in 9804ms
Creating board for 2021 12 6 Mon
Found 876 solutions in 30528ms
Creating board for 2021 12 7 Tue
Found 4364 solutions in 62080ms
Creating board for 2021 12 8 Wed
Found 1319 solutions in 32596ms
*/