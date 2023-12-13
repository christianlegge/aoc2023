#[test]
fn test() {
    solve(String::from(
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
    ));
}

struct Grid {
    rows: Vec<String>,
    cols: Vec<String>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(data: &str) -> Self {
        let mut rows = Vec::new();
        let mut cols = vec![String::new(); data.lines().next().unwrap().len()];

        for line in data.lines() {
            rows.push(String::from(line));
            for (idx, char) in line.chars().enumerate() {
                cols[idx].push(char);
            }
        }

        Grid {
            height: rows.len(),
            width: cols.len(),
            rows,
            cols,
        }
    }

    fn print(&self) {
        println!("{}", self.rows.join("\n"));
    }

    fn get_mirror_lines(&self) -> MirrorLines {
        let horizontal_axis = find_mirror_point(self.rows.as_slice());
        let vertical_axis = find_mirror_point(self.cols.as_slice());

        if horizontal_axis > 0 {
            dbg!(MirrorLines::Horizontal(horizontal_axis))
        } else if vertical_axis > 0 {
            dbg!(MirrorLines::Vertical(vertical_axis))
        } else {
            self.print();
            panic!("axis not found^");
        }
    }
}

fn find_mirror_point(lines: &[String]) -> usize {
    let mut axis = 1;
    while axis < lines.len() {
        let mut mirrored_lines = 0;
        let (mut left, mut right): (i32, usize) =
            ((axis - 1 - mirrored_lines) as i32, axis + mirrored_lines);
        while left >= 0 && right < lines.len() {
            println!(
                "checking equality:\n[{}] {}\n[{}] {}",
                left, lines[left as usize], right, lines[right]
            );
            if lines[left as usize] == lines[right] {
                println!("true");
                mirrored_lines += 2;
                left -= 1;
                right += 1;
            } else {
                println!("false");
                break;
            }
        }
        if !(left >= 0 && right < lines.len()) {
            println!("mirror axis found: {}", axis);
            return axis;
        }
        axis += 1;
    }
    0
}

#[derive(Debug)]
enum MirrorLines {
    Horizontal(usize),
    Vertical(usize),
}

pub fn solve(data: String) {
    println!("{}", data);
    let patterns = data.split("\n\n");
    let mut sum = 0;
    for pattern in patterns {
        let grid = Grid::new(pattern);
        match grid.get_mirror_lines() {
            MirrorLines::Vertical(n) => sum += n,
            MirrorLines::Horizontal(n) => sum += 100 * n,
        }
        grid.print();
    }
    println!("sum: {}", sum);
}
