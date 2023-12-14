#[test]
fn test() {
    solve(String::from(
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
    ));
}

struct RockGrid {
    walls: Vec<(usize, usize)>,
    width: usize,
    height: usize,
    rocks: Vec<(usize, usize)>,
}

impl RockGrid {
    fn new(data: String) -> Self {
        let height = data.lines().count();
        let width = data.lines().next().unwrap().len();
        let mut walls = Vec::new();
        let mut rocks = Vec::new();
        for (row, line) in data.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                match char {
                    '#' => walls.push((row, col)),
                    'O' => rocks.push((row, col)),
                    _ => continue,
                }
            }
        }
        RockGrid {
            walls,
            width,
            height,
            rocks,
        }
    }

    fn sort_rocks(&mut self) {
        self.rocks
            .sort_by(|(row, _col), (row2, _col2)| row2.partial_cmp(row).unwrap());
    }

    fn shift_north(&mut self) {
        let mut new_rocks = Vec::new();
        while let Some((row, col)) = self.rocks.pop() {
            let mut wall_row = row as i32;
            let new_rock = (
                loop {
                    wall_row -= 1;
                    if wall_row == -1
                        || self.walls.contains(&(wall_row as usize, col))
                        || new_rocks.contains(&(wall_row as usize, col))
                    {
                        break (wall_row + 1) as usize;
                    }
                },
                col,
            );
            // println!("rock at {}, {} moved to row {}", row, col, new_rock.0);
            new_rocks.push(new_rock);
        }
        self.rocks = new_rocks;
    }

    fn calculate_load(&self) -> usize {
        let mut sum = 0;
        for (row, _) in &self.rocks {
            sum += self.height - row;
        }
        // dbg!(&self.rocks);
        sum
    }

    fn print_grid(&self) {
        let mut grid = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                grid.push(
                    match (
                        self.rocks.contains(&(row, col)),
                        self.walls.contains(&(row, col)),
                    ) {
                        (true, true) => 'X',
                        (true, false) => 'O',
                        (false, true) => '#',
                        (false, false) => '.',
                    },
                );
            }
            grid.push('\n');
        }
        println!("grid:\n{}\n", grid);
    }
}

pub fn solve(data: String) {
    // println!("{}", data);
    let mut grid = RockGrid::new(data);
    grid.print_grid();
    grid.sort_rocks();
    // grid.shift_north();
    // grid.print_grid();
    for i in 0..1000000000 {
        // if i % 10000 == 0 {
        println!("iteration {}", i);
        // }
        grid.shift_north();
    }

    dbg!(grid.calculate_load());
}
