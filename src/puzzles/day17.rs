use std::collections::HashMap;

#[test]
fn test() {
    solve(String::from(
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533",
    ));
}

#[derive(Eq, PartialEq, Debug, Hash)]
enum Direction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Coords {
    row: usize,
    col: usize,
}

#[derive(Debug)]
struct CrucibleMap {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct AStarScores {
    parent: Coords,
    dir: Direction,
    g: usize,
    h: usize,
}

impl CrucibleMap {
    fn new(data: String) -> Self {
        let mut grid = Vec::new();
        for line in data.lines() {
            grid.push(Vec::new());
            for char in line.chars() {
                grid.last_mut().unwrap().push(char);
            }
        }
        CrucibleMap {
            width: grid.first().unwrap().len(),
            height: grid.len(),
            grid,
        }
    }

    fn find_path(&self) -> usize {
        let mut open = HashMap::new();
        let mut closed = HashMap::new();
        let mut curr = Coords { row: 0, col: 0 };
        let mut curr_scores = AStarScores {
            g: 0,
            h: self.heuristic(curr),
            dir: Direction::Right(0),
            parent: curr,
        };
        open.insert(curr, curr_scores);
        loop {
            if open.len() == 0 {
                panic!("unable to find path");
            }
            let mut kvs = open.iter().collect::<Vec<_>>();
            kvs.sort_by(|(k1, v1), (k2, v2)| )
            let valid_next = self.find_valid_next(curr, curr_scores.dir);
        }
        0
    }

    fn find_valid_next(&self, curr: Coords, dir: Direction) -> Vec<Coords> {
        let mut coords = Vec::new();
        if curr.row > 0 && dir != Direction::Up(3) && !matches!(dir, Direction::Down(_)) {
            coords.push(Coords {
                row: curr.row - 1,
                col: curr.col,
            })
        }
        if curr.row < self.height - 1
            && dir != Direction::Down(3)
            && !matches!(dir, Direction::Up(_))
        {
            coords.push(Coords {
                row: curr.row + 1,
                col: curr.col,
            })
        }
        if curr.col > 0 && dir != Direction::Left(3) && !matches!(dir, Direction::Right(_)) {
            coords.push(Coords {
                row: curr.row,
                col: curr.col - 1,
            })
        }
        if curr.col < self.width - 1
            && dir != Direction::Right(3)
            && !matches!(dir, Direction::Left(_))
        {
            coords.push(Coords {
                row: curr.row,
                col: curr.col + 1,
            })
        }
        coords
    }

    fn heuristic(&self, coords: Coords) -> usize {
        (self.height - coords.row) + (self.width - coords.col)
    }
}
pub fn solve(data: String) {
    let grid = CrucibleMap::new(data);
    dbg!(grid.find_valid_next(Coords { row: 0, col: 0 }, Direction::Right(0)));
}
