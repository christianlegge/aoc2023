#[test]
fn test() {
    solve(String::from(
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
    ));
}
struct Ranges {
    dest: i64,
    src: i64,
    len: i64,
}

struct SeedMap {}

struct Maps {}

pub fn solve(data: String) {
    let mut lines = data.split("\n");
    let seeds = get_seeds(lines.next().unwrap());
    let mut maps = Vec::new();
    for line in lines {
        if line.contains("map:") {
            maps.push(Vec::new());
        } else if line.trim() == "" {
            continue;
        } else {
            maps.last_mut().unwrap().push(get_ranges(line));
        }
    }
    let ends = seeds.into_iter().map(|n| walk_maps(n, &maps));
    println!("min location: {}", ends.min().unwrap());
}

fn get_seeds(seed_line: &str) -> Vec<i64> {
    let parts = seed_line.split_whitespace();
    let mut nums = Vec::new();
    for part in parts {
        if let Ok(n) = part.parse::<i64>() {
            nums.push(n);
        }
    }
    nums
}

fn walk_maps(seed: i64, maps: &Vec<Vec<Ranges>>) -> i64 {
    let mut curr = seed;
    'outer: for map in maps {
        for range in map {
            if range.src <= curr && curr < range.src + range.len {
                curr = range.dest + (curr - range.src);
                continue 'outer;
            }
        }
    }
    curr
}

fn get_ranges(number_line: &str) -> Ranges {
    let nums = number_line.split_whitespace();
    let nums_parsed = nums
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    Ranges {
        dest: nums_parsed[0],
        src: nums_parsed[1],
        len: nums_parsed[2],
    }
}
