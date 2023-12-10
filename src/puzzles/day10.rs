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

enum Direction {
    Left,
    Right,
    Up,
    Down,
}
impl PipeMap {
    fn walk_path(&self) -> usize {
        let mut length = 0;
        let mut next = if self.start.x > 0 && self.tiles[self.start.y][self.start.x - 1] == 'F'
            || self.tiles[self.start.y][self.start.x - 1] == '7'
        {
            Coords {
                x: self.start.x - 1,
                y: self.start.y,
            }
        } else if self.start.x < self.width - 1 && self.tiles[self.start.y][self.start.x + 1] == 'F'
            || self.tiles[self.start.y][self.start.x + 1] == 'L'
        {
            Coords {
                x: self.start.x + 1,
                y: self.start.y,
            }
        } else if self.start.y > 0 && self.tiles[self.start.y - 1][self.start.x];
    }

    fn find_next(&self, from_dir: Direction, cur_tile: Coords) -> Coords {
        let tile_value = self.tiles[cur_tile.y][cur_tile.x];
        match (from_dir, tile_value) {
            (Left, '7') => Coords {
                x: cur_tile.x,
                y: cur_tile.y + 1,
            },
            (Left, 'J') => Coords {
                x: cur_tile.x,
                y: cur_tile.y - 1,
            },
            (Right, 'L') => Coords {
                x: cur_tile.x,
                y: cur_tile.y - 1,
            },
            (Right, 'F') => Coords {
                x: cur_tile.x,
                y: cur_tile.y + 1,
            },
            (Up, 'L') => Coords {
                x: cur_tile.x + 1,
                y: cur_tile.y,
            },
            (Up, 'J') => Coords {
                x: cur_tile.x - 1,
                y: cur_tile.y,
            },
            (Down, '7') => Coords {
                x: cur_tile.x - 1,
                y: cur_tile.y,
            },
            (Down, 'F') => Coords {
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
