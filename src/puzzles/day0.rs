pub fn solve(data: String) {
    println!("Text input: {}", data);
    let data = data.split("\n");
    let mut maxSum = 0;
    let mut sum = 0;
    for line in data {
        println!("Line: {}", line);
        if line.trim() == "" {
            println!("Sum: {}", sum);
            if sum > maxSum {
                maxSum = sum;
            }
            sum = 0;
        } else {
            let num = line.parse::<i32>();
            match num {
                Ok(num) => {
                    sum += num;
                }
                Err(error) => {
                    println!("Error parsing number: {}", error);
                }
            }
        }
    }
    println!("Max sum: {}", maxSum);
}
