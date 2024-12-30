use std::fs;

fn calculate(numbers: &Vec<i32>, index: usize, sum: i64, wanted_result: i64) -> bool {
    let current_number: i64 = numbers[index] as i64;
    if numbers.len() - 1 == index {
        wanted_result == sum * current_number || wanted_result == sum + current_number
    } else {
        calculate(numbers, index + 1, sum + current_number, wanted_result)
            || calculate(numbers, index + 1, sum * current_number, wanted_result)
    }
}

pub fn part_one() {
    let input: String = fs::read_to_string("./src/day_seven/input.txt")
        .expect("Should have been able to read the file");

    let mut sum: i64 = 0;
    for line in input.lines() {
        let strings: Vec<&str> = line.split(':').collect();
        let result: i64 = strings[0].parse().unwrap();
        let mut numbers: Vec<i32> = Vec::new();
        for string in strings[1].split_whitespace() {
            numbers.push(string.parse().unwrap());
        }
        if calculate(&numbers, 1, numbers[0] as i64, result) {
            sum += result;
        }
    }
    println!("{}", sum)
}

fn calculate_two(numbers: &Vec<i32>, index: usize, sum: i64, wanted_result: i64) -> bool {
    let current_number: i64 = numbers[index] as i64;
    if numbers.len() - 1 == index {
        wanted_result == sum * current_number
            || wanted_result == sum + current_number
            || wanted_result == concat(sum, current_number as i64)
    } else {
        calculate_two(numbers, index + 1, sum + current_number, wanted_result)
            || calculate_two(numbers, index + 1, sum * current_number, wanted_result)
            || calculate_two(
                numbers,
                index + 1,
                concat(sum, current_number),
                wanted_result,
            )
    }
}

fn concat(first_number: i64, second_number: i64) -> i64 {
    let t = first_number.to_string() + &second_number.to_string();
    t.parse::<i64>().unwrap()
}

pub fn part_two() {
    let input: String = fs::read_to_string("./src/day_seven/input.txt")
        .expect("Should have been able to read the file");

    let mut sum: i64 = 0;
    for line in input.lines() {
        let strings: Vec<&str> = line.split(':').collect();
        let result: i64 = strings[0].parse().unwrap();
        let mut numbers: Vec<i32> = Vec::new();
        for string in strings[1].split_whitespace() {
            numbers.push(string.parse().unwrap());
        }
        if calculate_two(&numbers, 1, numbers[0] as i64, result) {
            sum += result;
        }
    }
    println!("{}", sum)
}
