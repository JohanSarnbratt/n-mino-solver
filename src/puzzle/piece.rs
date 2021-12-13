use std;
use std::fmt::Formatter;

struct Piece {
    coords: Vec<(i32, i32)>
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use std::cmp::max;
        let maxs = self.coords.iter().fold((0i32,0i32), |m: (i32, i32), p: &(i32, i32)| { (max(m.0, p.0), max(m.1, p.1)) });
        let str: String = (0..((maxs.0+1) * (maxs.1+1))).map(|ind| {
            let x = ind%(maxs.0+1);
            let y = ind/(maxs.0+1);
            [if self.coords.contains(&(x, y)) {
                "#"
            } else {
                "'"
            }, if x == maxs.0 {"\n"} else { "" }].join("")
        }).collect();
        write!(f, "{}",str)
    }
}

pub fn pieces() {
    let p1 = Piece { coords: vec![(0, 0), (1, 0), (0, 1), (1, 1), (2, 0)] };
    let p2 = Piece { coords: vec![(0, 0), (1, 0), (0, 1), (2, 0)] };
    std::println!("p1 is \n{}", p1);
    std::println!("p2 is \n{}", p2);
}