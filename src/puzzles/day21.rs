use std::collections::HashSet;

#[test]
fn test() {
    solve(String::from(""));
}

struct Garden {
    grid: Vec<Vec<char>>,
    reachable: HashSet<(usize, usize)>,
    height: usize,
    width: usize,
}

impl Garden {
    fn new(data: &str) -> Self {
        let mut rows = Vec::new();
        let mut reachable = HashSet::new();
        for (ridx, line) in data.lines().enumerate() {
            let mut row = Vec::new();
            for (cidx, char) in line.chars().enumerate() {
                row.push(char);
                if char == 'S' {
                    reachable.insert((ridx, cidx));
                }
            }
            rows.push(row);
        }
        Garden {
            width: rows.first().unwrap().len(),
            height: rows.len(),
            reachable,
            grid: rows,
        }
    }

    fn walk(&mut self, steps: usize) -> usize {
        if steps == 0 {
            return self.reachable.len();
        }

        let mut new_tiles = Vec::new();

        for (row, col) in &self.reachable {
            if *row > 0 && self.grid[row - 1][*col] == '.' {
                new_tiles.push((row - 1, *col));
            }
            if *row < self.height - 1 && self.grid[row + 1][*col] == '.' {
                new_tiles.push((row + 1, *col));
            }
            if *col > 0 && self.grid[*row][col - 1] == '.' {
                new_tiles.push((*row, col - 1));
            }
            if *col < self.width - 1 && self.grid[*row][col + 1] == '.' {
                new_tiles.push((*row, col + 1));
            }
        }

        self.reachable.extend(new_tiles.iter());

        self.walk(steps - 1)
    }
}

pub fn solve(data: String) {
    let mut garden = Garden::new(data.as_str());
    println!("{}", garden.walk(64));
}
