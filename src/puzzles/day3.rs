pub fn solve(data: String) {
    println!("{}, {}", data, parse_num(&data));
}

fn find_symbols(str: &str) -> Vec<usize> {
    let mut idxs = Vec::new();
    for (idx, char) in str.char_indices() {
        if char != '.' && !char.is_digit(10) {
            idxs.push(idx);
        }
    }
    idxs
}

struct NumInfo {
    num: u32,
    len: usize,
}
fn parse_num(str: &str) -> NumInfo {
    let mut num = 0;
    let mut len = 0;
    for char in str.chars() {
        if let Some(d) = char.to_digit(10) {
            num *= 10;
            num += d;
            len += 1;
        } else {
            return NumInfo { num, len };
        }
    }
    NumInfo { num, len }
}

fn find_valid_numbers(before: Vec<usize>, line: &str, after: Vec<usize>) -> Vec<u32> {
    let cur_symbols = find_symbols(line);
    let mut valid_numbers = Vec::new();
    for (idx, char) in line.char_indices() {
        if char.is_digit(10) {
            let NumInfo { num, len } = parse_num(&line[idx..]);
            let valid_indices = idx - 1..idx + (len);
            for valid_idx in valid_indices {
                if before.contains(&valid_idx)
                    || cur_symbols.contains(&valid_idx)
                    || after.contains(&valid_idx)
                {
                    valid_numbers.push(num);
                }
            }
        }
    }
    valid_numbers
}
