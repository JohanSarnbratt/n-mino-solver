use crate::puzzle::piece::{Piece, pieces_board_4};

#[derive(Clone, PartialEq)]
enum BoardElement {
    Wall,
    Empty,
    Piece(char)
}

pub struct Board {
    pub width: i32,
    pub height: i32,
    elements: Vec<BoardElement>,
    pub name: String
}

impl Board {
    fn valid(&self) -> bool {
        self.elements.len() == (self.width * self.height) as usize
    }
    fn full(&self) -> bool {
        !self.elements.contains(&BoardElement::Empty)
    }
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.elements.get((x+y*self.width) as usize) {
                    Some(BoardElement::Wall) => print!("#"),
                    Some(BoardElement::Empty) => print!("`"),
                    Some(BoardElement::Piece(char)) => print!("{}", char),
                    None => print!("error")
                }
            }
            println!()
        }
    }
    pub fn place_piece(&self, piece: &Piece, x: i32, y: i32) -> Option<Board> {
        if self.validate_place_piece(piece, x, y) {
            let mut mut_elements: Vec<BoardElement> = self.elements.clone();
            for (xo,yo) in &piece.coords {
                mut_elements[self.get_board_index(x+xo, y+yo)] = BoardElement::Piece(piece.name);
            }
            return Some(Board {
                width: self.width,
                height: self.height,
                elements: mut_elements,
                name: self.name.clone()
            })
        }
        None
    }
    fn validate_place_piece(&self, piece: &Piece, x: i32, y: i32) -> bool {
        if piece.max_x + x >= self.width || piece.max_y + y >= self.height {
            return false;
        }
        for (xo,yo) in &piece.coords {
            if self.get_board_pos(x+xo, y+yo) != Some(&BoardElement::Empty) {
                return false;
            }
        }
        true
    }
    fn get_board_pos(&self, x: i32, y: i32 ) -> Option<&BoardElement> {
        self.elements.get((x+y*self.width) as usize)
    }
    fn get_board_index(&self, x: i32, y: i32 ) -> usize {
        (x+y*self.width) as usize
    }
    pub fn find_available_space(&self, remaining_pieces: &[Piece]) -> bool {
        let mut sizes: Vec<usize> = remaining_pieces.iter().map(|p: &Piece| -> usize {p.coords.len()}).collect();
        sizes.sort();
        find_available_space(&self.elements, &sizes, &(self.width as usize))
    }
}

fn find_available_space(elems: &Vec<BoardElement>, sizes: &Vec<usize>, width: &usize) -> bool {
    let size = elems.len();
    let mut spaces: Vec<Vec<usize>> = vec![];
    for i in 0..size {
        if elems[i] == BoardElement::Empty && !vec_vec_contains(&spaces, &i) {
            let new_space = get_space(&elems,i,&width);
            if !can_be_built(new_space.len(), sizes) {
                return false;
            }
            spaces.push( new_space);
        }
    }
    //All spaces have a buildable size, so we can maybe put a piece in each
    true
}

// Answers the question: Can a subset of numbers in `sizes` sum to `space`
fn can_be_built(space: usize, sizes: &Vec<usize>) -> bool {
    //println!("{} {:?} ", space, sizes);
    let size_len = sizes.len();
    let ret: bool;
    if size_len == 0 {
         ret = false;
    } else if sizes[0] == space {
        ret = true;
    } else if sizes[0] > space {
        ret = false
    } else if size_len == 1 {
        ret = false;
    } else if can_be_built(space-sizes[0], & sizes.clone().split_off(1)) {
        ret = true
    } else if can_be_built(space, & sizes.clone().split_off(1)) {
        ret = true
    } else {
        ret = false
    };
    //println!("{}",ret);
    return ret;
}

fn vec_vec_contains(spaces: &Vec<Vec<usize>>, u: &usize) -> bool {
    spaces.iter().any(|space: &Vec<usize>| -> bool {
        return space.contains(&u);
    })
}

fn get_space(elems: &Vec<BoardElement>, start: usize, width: &usize) -> Vec<usize> {
    let mut space: Vec<usize> = vec![];
    let mut new_inds: Vec<usize> = vec![start];
    while !new_inds.is_empty() {
        let mut next_inds: Vec<usize> = vec![];
        for ni in &new_inds {
            if ni%width != 0 && elems[ni-1] == BoardElement::Empty && !next_inds.contains(&(ni-1)) && !space.contains(&(ni-1)) {
                next_inds.push(ni - 1);
            }
            if ni%width != width-1 && elems[ni+1] == BoardElement::Empty && !next_inds.contains(&(ni+1)) && !space.contains(&(ni+1)) {
                next_inds.push(ni + 1);
            }
            if ni >= width && elems[ni-width] == BoardElement::Empty && !next_inds.contains(&(ni-width)) && !space.contains(&(ni-width)) {
                next_inds.push(ni-width);
            }
            if ni+width < elems.len() && elems[ni+width] == BoardElement::Empty && !next_inds.contains(&(ni+width)) && !space.contains(&(ni+width)) {
                next_inds.push(ni+width);
            }
        }
        space.append(&mut new_inds);
        new_inds = next_inds;
    }
    space
}

pub fn test_board() {
    let b1 = board4();
    println!("is b1 valid: {}", b1.valid());
    println!("is b1 full: {}", b1.full());
    b1.print();
    let pieces = pieces_board_4();
    let b_placed = b1.place_piece(&pieces[0],1,0);
    println!("Placed a piece: ");
    for b in b_placed {
        b.print()
    }
    let b_place_turned = b1.place_piece(&pieces[0].turn(),0,1);
    println!("Placed a turned piece: ");
    for b in b_place_turned {
        b.print()
    }
    ()
}

pub fn board4() -> Board {
    let mut elements: Vec<BoardElement> = std::iter::repeat(BoardElement::Empty).take(16).collect();
    elements[0] = BoardElement::Wall;
    elements[1] = BoardElement::Wall;
    return Board {
        width: 4,
        height: 4,
        elements,
        name: "Four by four".parse().unwrap()
    };
}
pub fn board8() -> Board {
    let elements: Vec<BoardElement> = std::iter::repeat(BoardElement::Empty).take(64).collect();
    return Board {
        width: 8,
        height: 8,
        elements,
        name: "Original".parse().unwrap()
    };
}
pub fn board6() -> Board {
    let elements: Vec<BoardElement> = std::iter::repeat(BoardElement::Empty).take(36).collect();
    return Board {
        width: 6,
        height: 6,
        elements,
        name: "Original".parse().unwrap()
    };
}
pub fn date_board() -> Board {
    let mut elements: Vec<BoardElement> = std::iter::repeat(BoardElement::Empty).take(56).collect();
    elements[6] = BoardElement::Wall;
    elements[13] = BoardElement::Wall;
    elements[49] = BoardElement::Wall;
    elements[50] = BoardElement::Wall;
    elements[51] = BoardElement::Wall;
    elements[52] = BoardElement::Wall;
    //TODO base variable walls on actual date
    elements[12] = BoardElement::Wall; //December
    elements[29] = BoardElement::Wall; //16th
    elements[53] = BoardElement::Wall; //Thursday

    return Board {
        width: 7,
        height: 8,
        elements,
        name: "Original".parse().unwrap()
    };
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_can_be_built() {
        let should_be_false = can_be_built(14, &vec![4,5,6,7]);
        assert_eq!(should_be_false, false);
    }
    #[test]
    fn test_can_be_built2() {
        let should_be_true = can_be_built(12, &vec![4,5,6,7]);
        assert_eq!(should_be_true, true);
    }
}