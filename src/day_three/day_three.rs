use regex::{Match, Regex};
use std::fs;

pub fn day_three_part_one() {
    let input = fs::read_to_string("./src/day_three/input.txt")
        .expect("Should have been able to read the file");

    let regex = Regex::new(r"mul\(\d*,\d*\)").unwrap();
    let mut sum: i32 = 0;
    let match_vec: Vec<Match> = regex.find_iter(&input).collect();
    for string in match_vec {
        sum += calculate_sum(string.as_str())
    }
    println!("{}", sum);
}

pub fn day_three_part_two() {
    let input = fs::read_to_string("./src/day_three/input.txt")
        .expect("Should have been able to read the file");

    let regex = Regex::new(r"mul\(\d*,\d*\)|do\(\)|don't\(\)").unwrap();
    let mut sum: i32 = 0;
    let match_vec: Vec<Match> = regex.find_iter(&input).collect();
    let mut enabled: bool = true;
    for matched_string in match_vec {
        let string = matched_string.as_str();
        match string {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    sum += calculate_sum(string)
                }
            }
        }
    }
    println!("{}", sum);
}

fn calculate_sum(string: &str) -> i32 {
    let fixed_string: String = string.replace(&['m', 'u', 'l', '(', ')'][..], "");
    let mut inner_sum: i32 = 1;
    for number_string in fixed_string.split(",") {
        let number: i32 = number_string.parse().unwrap();
        inner_sum = inner_sum * number;
    }
    inner_sum
}
