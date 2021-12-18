use std;
use std::fmt::Formatter;

pub struct Piece {
    pub coords: Vec<(usize, usize)>,
    pub max_x: usize,
    pub max_y: usize,
    pub name: char,
    pub all_perms: Vec<Piece>
}

impl Piece {
    pub fn turn(&self) -> Piece {
        let new_coords = self.coords.iter().map(|(x,y)| -> (usize,usize) { return (self.max_y - *y, *x); }).collect();
        Piece {
            coords: new_coords,
            max_x: self.max_y,
            max_y: self.max_x,
            name: self.name,
            all_perms: vec![]
        }
    }
    pub fn mirror(&self) -> Piece {
        let new_coords = self.coords.iter().map(|(x,y)| -> (usize,usize) { return (*x, self.max_y - *y); }).collect();
        Piece {
            coords: new_coords,
            max_x: self.max_x,
            max_y: self.max_y,
            name: self.name,
            all_perms: vec![]
        }
    }
}
fn generate_all_perms(piece: &Piece) -> Vec<Piece> {
    let p1 = piece.turn();
    let p2 = p1.turn();
    let p3 = p2.turn();
    let p0 = p3.turn();
    let p4 = piece.mirror();
    let p5 = p4.turn();
    let p6 = p5.turn();
    let p7 = p6.turn();
    let mut perms: Vec<Piece> = vec![p0, p1, p2, p3, p4, p5, p6, p7];
    perms.iter_mut().for_each(| p| {
        p.coords.sort();
    });
    perms.sort_by(|pa: &Piece, pb: &Piece| { pa.coords.cmp(&pb.coords) });
    perms.dedup_by(|pa, pb| {
        return pa.coords == pb.coords;
    });
    return perms;
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use std::cmp::max;
        let maxs = self.coords.iter().fold((0,0), |m: (usize, usize), p: &(usize, usize)| { (max(m.0, p.0), max(m.1, p.1)) });
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

pub fn construct_piece(coords: Vec<(usize, usize)>, name: char) -> Piece {
    use std::cmp::max;
    let maxs = coords.iter().fold((0usize,0usize), |m: (usize, usize), p: &(usize, usize)| { (max(m.0, p.0), max(m.1, p.1)) });
    let piece_without_perms = Piece { coords: coords.clone(), name, max_x: maxs.0, max_y: maxs.1, all_perms: vec![]};
    let perms = generate_all_perms(&piece_without_perms);
    return Piece { coords, name, max_x: maxs.0, max_y: maxs.1, all_perms: perms};
}

pub fn pieces_board_4() -> Vec<Piece> {
    let p1 = piece_t();
    let p2 = piece_z();
    let p3 = piece_v();
    return vec![p1, p2, p3]
}
pub fn test_pieces() {
    let pieces = pieces_board_4();
    std::println!("p1 is \n{}", pieces[0]);
    std::println!("p2 is \n{}", pieces[1]);
    std::println!("p2 mirror is \n{}", pieces[1].mirror());
    for piece in pieces {
        let perms = piece.all_perms;
        println!("All {} perms:", perms.len());
        for p in perms {
            println!("{}", p);
        }
    }
}

fn piece_t() -> Piece {
    return construct_piece(vec![(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)],  'T');
}

fn piece_l() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (1, 0)], 'L');
}
fn piece_long_l() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0)], 'K');
}
fn piece_z() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (1, 1), (1, 2)], 'z');
}
fn piece_big_z() -> Piece {
    return construct_piece(vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2), (3, 2), (4, 2)], 'Z');
}
fn piece_medium_z() -> Piece {
    return construct_piece(vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)], 'Z');
}

fn piece_v() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (1, 0), (2, 0)], 'v');
}
fn piece_long_v() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (1, 0), (2, 0), (3, 0)], 'V');
}
fn piece_long_p() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (1, 3), (1, 4)], 'P');
}
fn piece_short_p() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 2), (1, 3)], 'p');
}
fn piece_mini_p() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (1, 1), (1, 2)], 'q');
}
fn piece_fat_d() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 2), (2, 3)], 'd');
}
fn piece_q() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1)], 'Q');
}
fn piece_box() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2), (1, 3)], 'B');
}
fn piece_line() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (0, 3)], 'I');
}
fn piece_u() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2)], 'U');
}
fn piece_r() -> Piece {
    return construct_piece(vec![(0, 0), (0, 1), (0, 2), (1, 2), (1, 3)], 'R');
}

pub fn pieces_for_original() -> Vec<Piece> {
    return vec![
        piece_long_p(),
        piece_short_p(),
        piece_mini_p(),
        piece_fat_d(),
        piece_v(),
        piece_long_v(),
        piece_big_z(),
        piece_l(),
        piece_q(),
        piece_box()
    ];
}
pub fn pieces_for_small() -> Vec<Piece> {
    return vec![
        piece_long_p(), //7
        piece_short_p(), //6
        piece_mini_p(), // 5
        piece_fat_d(), //8
        piece_v(), //5
        piece_long_l(), //5
    ];
}
pub fn pieces_for_date() -> Vec<Piece> {
    return vec![
        piece_v(),
        piece_mini_p(),
        piece_long_l(),
        piece_l(),
        piece_z(),
        piece_line(),
        piece_medium_z(),
        piece_t(),
        piece_u(),
        piece_r(),
    ];
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn perms_of_pieces_consider_symetry() {
        let piece = construct_piece(vec![(0,0), (1,0), (0,1), (1,1)], '#');
        assert_eq!(piece.all_perms().len(), 1);
    }
    #[test]
    fn perms_of_pieces_consider_symetry2() {
        let piece = construct_piece(vec![(0,0), (1,0), (0,1), (2,0)], 'L');
        assert_eq!(piece.all_perms().len(), 8);
    }
}
