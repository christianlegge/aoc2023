use std::collections::HashSet;

#[test]
fn test() {
    solve(String::from(
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
    ));
}

#[derive(Copy, Clone)]
struct Coords {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn bounce(&self, c: char) -> Vec<Self> {
        use Direction::*;
        match (self, c) {
            (_, '.') => vec![self.clone()],
            (Up, '|') => vec![Up],
            (Down, '|') => vec![Down],
            (Left, '|') => vec![Up, Down],
            (Right, '|') => vec![Up, Down],
            (Up, '-') => vec![Left, Right],
            (Down, '-') => vec![Left, Right],
            (Left, '-') => vec![Left],
            (Right, '-') => vec![Right],
            (Up, '/') => vec![Right],
            (Down, '/') => vec![Left],
            (Left, '/') => vec![Up],
            (Right, '/') => vec![Down],
            (Up, '\\') => vec![Left],
            (Down, '\\') => vec![Right],
            (Left, '\\') => vec![Down],
            (Right, '\\') => vec![Up],
            _ => panic!("unknown character in grid"),
        }
    }

    fn next(&self, loc: Coords) -> Coords {
        match self {
            Direction::Up => Coords {
                row: loc.row,
                col: loc.col - 1,
            },
            Direction::Down => Coords {
                row: loc.row,
                col: loc.col + 1,
            },
            Direction::Left => Coords {
                row: loc.row - 1,
                col: loc.col,
            },
            Direction::Right => Coords {
                row: loc.row + 1,
                col: loc.col,
            },
        }
    }
}

struct MirrorGrid {
    grid: Vec<Vec<char>>,
    glowing: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl MirrorGrid {
    fn new(data: &str) -> Self {
        let mut lines = Vec::new();
        for _ in data.lines() {
            lines.push(Vec::new());
            for char in data.chars() {
                lines.last_mut().unwrap().push(char);
            }
        }
        MirrorGrid {
            grid: lines.clone(),
            glowing: HashSet::new(),
            width: lines.first().unwrap().len(),
            height: lines.len(),
        }
    }

    fn propagate_ray(&mut self, dir: Direction, loc: (usize, usize)) {
        unimplemented!();
    }
}

pub fn solve(data: String) {
    println!("{}", data);
    for line in data.lines() {
        println!("{}", line);
    }
}
