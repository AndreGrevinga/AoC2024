use std::fs;
pub fn day_one_part_one() {
    let input = fs::read_to_string("./src/day_one/input.txt")
        .expect("Should have been able to read the file");

    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();
    let mut results: Vec<i32> = Vec::new();
    for (i, string) in parts.iter().enumerate() {
        let number: i32 = string.parse().unwrap();
        if i % 2 == 0 {
            left_vec.push(number);
        } else {
            right_vec.push(number);
        }
        left_vec.sort();
        right_vec.sort();
    }
    for (i, number) in left_vec.iter().enumerate() {
        let diff: i32 = number - right_vec[i];
        results.push(diff.abs());
    }
    let sum: i32 = results.iter().sum();
    println!("{}", sum.to_string());
}

pub fn day_one_part_two() {
    let input = fs::read_to_string("./src/day_one/input.txt")
        .expect("Should have been able to read the file");

    let parts: Vec<&str> = input.split_whitespace().collect();
    let mut left_vec: Vec<i32> = Vec::new();
    let mut right_vec: Vec<i32> = Vec::new();
    let mut results: Vec<i32> = Vec::new();
    for (i, string) in parts.iter().enumerate() {
        let number: i32 = string.parse().unwrap();
        if i % 2 == 0 {
            left_vec.push(number);
        } else {
            right_vec.push(number);
        }
    }
    for number in left_vec {
        let occurence_vec: Vec<&i32> = right_vec.iter().filter(|num| **num == number).collect();
        results.push(occurence_vec.len() as i32 * number);
    }
    let sum: i32 = results.iter().sum();
    println!("{}", sum.to_string());
}
