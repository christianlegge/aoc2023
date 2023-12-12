use core::panic;

#[test]
fn test() {
    solve(String::from(
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
    ));
}

#[derive(Debug)]
enum SpringRun {
    Operational(u32),
    Damaged(u32),
    Unknown(u32),
}

#[derive(Debug)]
struct SpringRow {
    spring_runs: Vec<SpringRun>,
    groups: Vec<u32>,
}

impl SpringRow {
    fn new(data: &str) -> Self {
        let mut parts = data.split_whitespace();
        let springs = parts.next().unwrap();
        let groups = parts.next().unwrap();

        let mut chars = springs.chars();
        let mut curr = chars.next().unwrap();
        let mut num = 1;
        let mut runs = Vec::new();
        for char in chars {
            if char == curr {
                num += 1;
            } else {
                runs.push(match curr {
                    '.' => SpringRun::Operational(num),
                    '#' => SpringRun::Damaged(num),
                    '?' => SpringRun::Unknown(num),
                    _ => panic!("Encountered unknown spring character while parsing row"),
                });
                num = 1;
            }
            curr = char;
        }
        runs.push(match curr {
            '.' => SpringRun::Operational(num),
            '#' => SpringRun::Damaged(num),
            '?' => SpringRun::Unknown(num),
            _ => panic!("Encountered unknown spring character while parsing row"),
        });

        SpringRow {
            spring_runs: runs,
            groups: groups
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>(),
        }
    }
}

pub fn solve(data: String) {
    println!("{}", data);
    for line in data.lines() {
        let row = SpringRow::new(line);
        dbg!(row);
    }
}
