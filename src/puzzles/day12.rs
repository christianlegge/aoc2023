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
#[test]
fn test2() {
    dbg!(count_valid_configurations("?###???????? 3,2,1"));
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
    // springs: Vec<char>,
    groups: Vec<u32>,
}

impl SpringRow {
    fn new(data: &str) -> Self {
        let mut parts = data.split_whitespace();
        let springs = parts.next().unwrap();
        let groups = parts.next().unwrap();

        let mut spring_chars = Vec::new();

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
            spring_chars.push(char);
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
            // springs: spring_chars,
            groups: groups
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect::<Vec<u32>>(),
        }
    }

    fn is_valid(&self) -> Option<bool> {
        let damaged_runs = self
            .spring_runs
            .iter()
            .filter(|c| matches!(c, SpringRun::Damaged(_)))
            .map(|c| {
                if let SpringRun::Damaged(n) = c {
                    n
                } else {
                    panic!("error in is_valid finding damaged_runs")
                }
            });
        if damaged_runs.clone().count() == self.groups.len()
            && damaged_runs.zip(&self.groups).all(|(a, b)| a == b)
        {
            Some(true)
        } else {
            if self
                .spring_runs
                .iter()
                .any(|c| matches!(c, SpringRun::Unknown(_)))
            {
                None
            } else {
                Some(false)
            }
        }
    }
}

fn count_valid_configurations(data: &str) -> usize {
    // println!("counting valid configurations for {}", data);
    let row = SpringRow::new(data);
    match row.is_valid() {
        Some(valid) => {
            if valid {
                1
            } else {
                0
            }
        }
        None => {
            let parts = data.splitn(2, "?");
            // let next_row = remove_first_question(data);
            count_valid_configurations(&format!(
                "{}.{}",
                parts.clone().next().unwrap(),
                parts.clone().nth(1).unwrap()
            )) + count_valid_configurations(&format!(
                "{}#{}",
                parts.clone().next().unwrap(),
                parts.clone().nth(1).unwrap()
            ))
        }
    }
}

// fn remove_first_question(data: &str) -> String {
//     data.chars()
//         .skip_while(|c| c != &'?')
//         .skip(1)
//         .collect::<String>()
// }

pub fn solve(data: String) {
    println!("{}", data);
    let mut sum = 0;
    for line in data.lines() {
        // let row = SpringRow::new(line);
        // let mut parts = line.split_whitespace();
        // let springs = parts.next().unwrap().repeat(5);
        // let groups = parts.next().unwrap().repeat(5);
        // let configs = count_valid_configurations(&format!("{} {}", springs, groups));
        let configs = count_valid_configurations(line);
        // dbg!(row);
        println!("valid configurations for {}: {}", line, configs);
        sum += configs;
    }
    println!("total configs: {}", sum);
}
