use std;
use std::fmt::Formatter;

struct Piece {
    coords: Vec<(i32, i32)>
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.coords.len())
    }
}

pub fn pieces() {
    let p1 = Piece { coords: vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 0)] };
    let p2 = Piece { coords: vec![(0, 0), (1, 0), (0, 1), (2, 0)] };
    std::println!("p1 is {}", p1.coords.len());
    std::println!("p2 is {}", p2);
}