use std::fs;

fn get_position(vec: &Vec<i32>, number: i32) -> usize {
    vec.iter().position(|&num| num == number).unwrap()
}

fn load_rules() -> Vec<Vec<i32>> {
    let rule_file: String = fs::read_to_string("./src/day_five/rules.txt")
        .expect("Should have been able to read the file");
    let rules: Vec<Vec<i32>> = rule_file
        .lines()
        .map(|str| str.split('|').map(|i| i.parse::<i32>().unwrap()).collect())
        .collect::<Vec<Vec<i32>>>();
    rules
}

fn check_numbers(rules: &Vec<Vec<i32>>, numbers: &Vec<i32>) -> bool {
    let mut check_passed = true;
    for rule_vec in rules {
        let first_rule_num = rule_vec[0];
        let second_rule_num = rule_vec[1];
        if numbers.contains(&first_rule_num) && numbers.contains(&second_rule_num) {
            let pos_one: usize = get_position(numbers, first_rule_num);
            let pos_two: usize = get_position(numbers, second_rule_num);
            if pos_one > pos_two {
                check_passed = false;
                break;
            }
        }
    }
    check_passed
}

pub fn day_five_part_one() {
    let rules: Vec<Vec<i32>> = load_rules();
    let input: String = fs::read_to_string("./src/day_five/input.txt")
        .expect("Should have been able to read the file");

    let mut sum: i32 = 0;
    for line in input.lines() {
        let strings: Vec<&str> = line.split(',').collect();
        let mut numbers: Vec<i32> = Vec::new();

        for string in strings {
            numbers.push(string.parse().unwrap());
        }

        if check_numbers(&rules, &numbers) {
            sum += numbers[numbers.len() / 2];
        }
    }
    println!("{}", sum)
}
pub fn day_five_part_two() {
    let rules: Vec<Vec<i32>> = load_rules();
    let input: String = fs::read_to_string("./src/day_five/input.txt")
        .expect("Should have been able to read the file");

    let mut sum: i32 = 0;
    let mut counter = 0;
    for line in input.lines() {
        println!("{}", counter);
        counter += 1;
        let strings: Vec<&str> = line.split(',').collect();
        let mut numbers: Vec<i32> = Vec::new();

        for string in strings {
            numbers.push(string.parse().unwrap());
        }

        if !check_numbers(&rules, &numbers) {
            while !check_numbers(&rules, &numbers) {
                //Fix ordering
            }
            sum += numbers[numbers.len() / 2];
        }
    }
    println!("{}", sum)
}
