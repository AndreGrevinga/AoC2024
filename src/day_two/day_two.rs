use std::fs;

pub fn day_two_part_one() {
    let input = fs::read_to_string("./src/day_two/input.txt")
        .expect("Should have been able to read the file");

    let mut counter: i32 = 0;
    for line in input.lines() {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        let mut is_safe: bool = true;
        let mut previous_number: i32 = 0;
        let mut is_ascending: bool = true;
        for (i, string) in numbers.iter().enumerate() {
            let number: i32 = string.parse().unwrap();
            if !(previous_number == 0) {
                if previous_number == number || (previous_number - number).abs() > 3 {
                    is_safe = false;
                } else if number > previous_number {
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
            previous_number = number;
        }
        if is_safe {
            counter += 1;
        }
    }
    println!("{}", counter);
}

pub fn day_two_part_two() {
    let input1 = fs::read_to_string("./src/day_two/input.txt")
        .expect("Should have been able to read the file");

    let input2 = "1 2 3 4 5
  1 9 8 7 6
  9 1 8 7 6
  9 8 1 7 6
  9 8 7 1 6
  9 8 7 6 1
  9 1 2 3 4
  1 9 2 3 4
  1 2 9 3 4
  1 2 3 9 4
  1 2 3 4 9
  1 9 9 2
  1 3 1 3";

    let mut counter: i32 = 0;
    for line in input1.lines() {
        if check_number_string(line) {
            counter += 1;
        }
    }
    println!("{}", counter);
}

fn check_number_string(line_string: &str) -> bool {
    let numbers: Vec<&str> = line_string.split_whitespace().collect();
    let mut is_safe: bool = true;
    let mut previous_number: i32 = 0;
    let mut previous_previous_number: i32 = 0;
    let mut is_ascending: bool = true;
    let mut fail_safe_triggered: bool = false;
    let mut fail_safe: bool = true;
    for (i, string) in numbers.iter().enumerate() {
        if is_safe || fail_safe_triggered {
            let number: i32 = string.parse().unwrap();
            if !(i == 0) {
                if i == 1 {
                    is_ascending = number > previous_number;
                }
                if fail_safe_triggered {
                    if i == 2 {
                        //For third number check if first can be skipped
                        is_ascending = number > previous_number;
                        is_safe = check_numbers(number, previous_number, is_ascending);
                        if !is_safe {
                            //If not check if second can be skipped
                            is_ascending = number > previous_previous_number;
                            is_safe = check_numbers(number, previous_previous_number, is_ascending);
                        }
                    } else {
                        //For every other number check if previous can be skipped
                        is_safe = check_numbers(number, previous_previous_number, is_ascending);
                    }
                } else {
                    //Normal case without triggered fail safe
                    is_safe = check_numbers(number, previous_number, is_ascending);
                }
                if !is_safe && i + 1 == numbers.len() && fail_safe {
                    //Number is safe if last number is wrong but we have a fail safe
                    is_safe = true;
                }
                fail_safe_triggered = false;
                if !is_safe && fail_safe {
                    fail_safe_triggered = true;
                    fail_safe = false;
                }
            }
            previous_previous_number = previous_number;
            previous_number = number;
        }
    }
    println!("{:?}", line_string);
    println!("{}", is_safe);
    is_safe
}

fn check_numbers(first: i32, second: i32, ascending: bool) -> bool {
    let mut safe: bool = true;
    if (second == first) || (second - first).abs() > 3 {
        safe = false;
    } else if first > second && !ascending {
        safe = false;
    } else if first < second && ascending {
        safe = false;
    }
    safe
}
