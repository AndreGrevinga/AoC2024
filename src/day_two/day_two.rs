use std::fs;

pub fn day_two_part_one() {
    let input = fs::read_to_string("./src/day_two/input.txt")
        .expect("Should have been able to read the file");

    let mut counter: i32 = 0;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if check_number_line(&numbers) {
            counter += 1;
        }
    }
    println!("{}", counter);
}

pub fn day_two_part_two() {
    let input = fs::read_to_string("./src/day_two/input.txt")
        .expect("Should have been able to read the file");
    let mut counter: i32 = 0;
    for line in input.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let size = numbers.len();
        if check_number_line(&numbers) {
            counter += 1;
        } else {
            let mut check = false;
            for i in 0..size {
                let mut copy = numbers.clone();
                copy.remove(i);
                if check_number_line(&copy) {
                    check = true;
                }
            }
            if check {
                counter += 1;
            }
        }
    }
    println!("{}", counter);
}

fn check_number_line(numbers: &Vec<i32>) -> bool {
    let mut is_safe: bool = true;
    let mut previous_number: i32 = 0;
    let mut is_ascending: bool = true;
    for (i, num) in numbers.iter().enumerate() {
        if !(previous_number == 0) {
            if previous_number == *num || (previous_number - num).abs() > 3 {
                is_safe = false;
            } else if *num > previous_number {
                if i == 1 || is_ascending {
                    is_ascending = true;
                } else {
                    is_safe = false;
                }
            } else {
                if i == 1 || !is_ascending {
                    is_ascending = false;
                } else {
                    is_safe = false;
                }
            }
        }
        previous_number = *num;
    }
    is_safe
}
