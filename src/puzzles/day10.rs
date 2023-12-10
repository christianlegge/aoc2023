#[test]
fn test() {
    solve(String::from(
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
    ));
}

struct PipeMap {
    width: usize,
    height: usize,
    tiles: Vec<Vec<char>>,
    start: Coords,
}

struct Coords {
    x: usize,
    y: usize,
}

impl PartialEq for Coords {
    fn eq(&self, other: &Coords) -> bool {
        self.x == other.x && self.y == other.y
    }
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl PipeMap {
    fn walk_path(&self) -> usize {
        let mut length = 1;
        let (mut next, mut dir) = if self.start.x > 0
            && self.tiles[self.start.y][self.start.x - 1] == 'F'
            || self.tiles[self.start.y][self.start.x - 1] == '7'
        {
            (
                Coords {
                    x: self.start.x - 1,
                    y: self.start.y,
                },
                Direction::Right,
            )
        } else if self.start.x < self.width - 1 && self.tiles[self.start.y][self.start.x + 1] == 'F'
            || self.tiles[self.start.y][self.start.x + 1] == 'L'
        {
            (
                Coords {
                    x: self.start.x + 1,
                    y: self.start.y,
                },
                Direction::Left,
            )
        } else if self.start.y > 0 && self.tiles[self.start.y - 1][self.start.x] == 'F'
            || self.tiles[self.start.y - 1][self.start.x] == '7'
        {
            (
                Coords {
                    x: self.start.x,
                    y: self.start.y - 1,
                },
                Direction::Down,
            )
        } else if self.start.y < self.height - 1
            && self.tiles[self.start.y + 1][self.start.x] == 'J'
            || self.tiles[self.start.y + 1][self.start.x] == 'L'
        {
            (
                Coords {
                    x: self.start.x,
                    y: self.start.y + 1,
                },
                Direction::Up,
            )
        } else {
            panic!("error finding first tile");
        };
        while next != self.start {
            next = self.find_next(&dir, &next);
            length += 1;
        }
        length
    }

    fn find_next(&self, from_dir: &Direction, cur_tile: &Coords) -> Coords {
        let tile_value = self.tiles[cur_tile.y][cur_tile.x];
        match (from_dir, tile_value) {
            (Direction::Left, '7') => Coords {
                x: cur_tile.x,
                y: cur_tile.y + 1,
            },
            (Direction::Left, 'J') => Coords {
                x: cur_tile.x,
                y: cur_tile.y - 1,
            },
            (Direction::Right, 'L') => Coords {
                x: cur_tile.x,
                y: cur_tile.y - 1,
            },
            (Direction::Right, 'F') => Coords {
                x: cur_tile.x,
                y: cur_tile.y + 1,
            },
            (Direction::Up, 'L') => Coords {
                x: cur_tile.x + 1,
                y: cur_tile.y,
            },
            (Direction::Up, 'J') => Coords {
                x: cur_tile.x - 1,
                y: cur_tile.y,
            },
            (Direction::Down, '7') => Coords {
                x: cur_tile.x - 1,
                y: cur_tile.y,
            },
            (Direction::Down, 'F') => Coords {
                x: cur_tile.x + 1,
                y: cur_tile.y,
            },
            _ => panic!("invalid tile"),
        }
    }
}

pub fn solve(data: String) {
    let lines = data.split("\n");
    for line in lines {}
}
