use std;
use std::fmt::Formatter;

pub struct Piece {
    pub coords: Vec<(i32, i32)>,
    max_x: i32,
    max_y: i32,
    pub name: char
}

impl Piece {
    fn turn(&self) -> Piece {
        let new_coords = self.coords.iter().map(|(x,y)| -> (i32,i32) { return (self.max_y - *y, *x); }).collect();
        Piece {
            coords: new_coords,
            max_x: self.max_y,
            max_y: self.max_x,
            name: self.name
        }
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use std::cmp::max;
        let maxs = self.coords.iter().fold((0i32,0i32), |m: (i32, i32), p: &(i32, i32)| { (max(m.0, p.0), max(m.1, p.1)) });
        let str: String = (0..((maxs.0+1) * (maxs.1+1))).map(|ind| {
            let x = ind%(maxs.0+1);
            let y = ind/(maxs.0+1);
            [if self.coords.contains(&(x, y)) {
                self.name.to_string()
            } else {
                "'".to_string()
            }, if x == maxs.0 {"\n".to_string()} else { "".to_string() }].join("")
        }).collect();
        write!(f, "{}",str)
    }
}

pub fn construct_piece(coords: Vec<(i32, i32)>, name: char) -> Piece {
    use std::cmp::max;
    let maxs = coords.iter().fold((0i32,0i32), |m: (i32, i32), p: &(i32, i32)| { (max(m.0, p.0), max(m.1, p.1)) });
    return Piece { coords, name, max_x: maxs.0, max_y: maxs.1};
}

pub fn pieces() -> Vec<Piece> {
    let p1 = piece_t();
    let p2 = piece_z();
    return vec![p1, p2]
}
pub fn test_pieces() -> Vec<Piece> {
    let p1 = piece_t();
    let p2 = piece_z();
    std::println!("p1 is \n{}", p1);
    std::println!("p2 is \n{}", p2);
    return vec![p1, p2]
}

fn piece_t() -> Piece {
    return construct_piece(vec![(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)],  'T');
}

fn piece_z() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (1, 1), (1, 2)], 'Z');
}
