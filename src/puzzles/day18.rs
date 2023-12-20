#[test]
fn test() {
    solve(String::from(
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)",
    ));
}

enum Move {
    U(usize),
    D(usize),
    L(usize),
    R(usize),
}

struct Dig {
    dig_move: Move,
    color: String,
}

struct DigPlot {
    grid: Vec<Vec<char>>,
    dig_plan: Vec<Dig>,
}

impl DigPlot {
    fn new(data: String) -> Self {
        let mut dig_plan = Vec::new();
        let (mut row, mut col): (i32, i32) = (0, 0);
        let (mut width, mut height) = (0, 0);
        for line in data.lines() {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            match parts[..] {
                ["U", n, c] => {
                    let n = n.parse().unwrap();
                    dig_plan.push(Dig {
                        color: c.to_owned(),
                        dig_move: Move::U(n),
                    });
                    row -= n as i32;
                }
                ["D", n, c] => {
                    let n = n.parse().unwrap();
                    dig_plan.push(Dig {
                        color: c.to_owned(),
                        dig_move: Move::D(n),
                    });
                    row += n as i32;
                }
                ["L", n, c] => {
                    let n = n.parse().unwrap();
                    dig_plan.push(Dig {
                        color: c.to_owned(),
                        dig_move: Move::L(n),
                    });
                    col -= n as i32;
                }
                ["R", n, c] => {
                    let n = n.parse().unwrap();
                    dig_plan.push(Dig {
                        color: c.to_owned(),
                        dig_move: Move::R(n),
                    });
                    col += n as i32;
                }
                _ => panic!("error parsing dig line"),
            }
            if row > height {
                height = row;
            }
            if col > width {
                width = col;
            }
        }
        if row < 0 {
            height += -row;
        }
        if col < 0 {
            width += -col;
        }
        let mut grid = Vec::new();
        for i in 0..height {
            let mut rv = Vec::new();
            for j in 0..width {
                rv.push('.');
            }
            grid.push(rv);
        }
        DigPlot { grid, dig_plan }
    }
}

pub fn solve(data: String) {
    for line in data.lines() {
        println!("{}", line);
    }
}
