use std;
use std::fmt::Formatter;

pub struct Piece {
    pub coords: Vec<(i32, i32)>,
    pub max_x: i32,
    pub max_y: i32,
    pub name: char
}

impl Piece {
    pub fn turn(&self) -> Piece {
        let new_coords = self.coords.iter().map(|(x,y)| -> (i32,i32) { return (self.max_y - *y, *x); }).collect();
        Piece {
            coords: new_coords,
            max_x: self.max_y,
            max_y: self.max_x,
            name: self.name
        }
    }
    pub fn mirror(&self) -> Piece {
        let new_coords = self.coords.iter().map(|(x,y)| -> (i32,i32) { return (*x, self.max_y - *y); }).collect();
        Piece {
            coords: new_coords,
            max_x: self.max_x,
            max_y: self.max_y,
            name: self.name
        }
    }
    pub fn all_perms(&self) -> [Piece; 8] { //todo check for duplicates
        let p1 = self.turn();
        let p2 = p1.turn();
        let p3 = p2.turn();
        let p0 = p3.turn();
        let p4 = self.mirror();
        let p5 = p4.turn();
        let p6 = p5.turn();
        let p7 = p6.turn();
        return [p0, p1, p2, p3, p4, p5, p6, p7];
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
    let p3 = piece_v();
    return vec![p1, p2, p3]
}
pub fn test_pieces() -> Vec<Piece> {
    let p1 = piece_t();
    let p2 = piece_z();
    std::println!("p1 is \n{}", p1);
    std::println!("p2 is \n{}", p2);
    std::println!("p2 mirror is \n{}", p2.mirror());
    return vec![p1, p2]
}

fn piece_t() -> Piece {
    return construct_piece(vec![(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)],  'T');
}

fn piece_z() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (1, 1), (1, 2)], 'Z');
}

fn piece_v() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (1, 0), (2, 0)], 'V');
}
