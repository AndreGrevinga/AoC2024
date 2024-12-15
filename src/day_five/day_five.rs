use std::fs;

fn get_position(vec: &Vec<i32>, number: i32) -> usize {
    vec.iter().position(|&num| num == number).unwrap()
}

fn load_rules() -> Vec<i32> {
    let mut rules: Vec<i32> = Vec::new();
    let rule_file: String = fs::read_to_string("./src/day_five/rules.txt")
        .expect("Should have been able to read the file");

    for line in rule_file.lines() {
        let numbers: Vec<&str> = line.split('|').collect();
        let first_number: i32 = numbers[0].parse().unwrap();
        let second_number: i32 = numbers[1].parse().unwrap();
        if rules.contains(&first_number) {
            if rules.contains(&second_number) {
                let pos_one = get_position(&rules, first_number);
                let pos_two = get_position(&rules, second_number);
                if pos_one > pos_two {
                    rules.remove(pos_two);
                    if pos_one >= rules.len() {
                        rules.push(second_number);
                    } else {
                        rules.insert(pos_one + 1, second_number);
                    }
                }
            } else {
                let pos: usize = get_position(&rules, first_number);
                if pos == rules.len() {
                    rules.push(second_number);
                } else {
                    rules.insert(pos + 1, second_number);
                }
            }
        } else {
            if rules.contains(&second_number) {
                let pos: usize = get_position(&rules, second_number);
                if pos == 0 {
                    rules.insert(pos, first_number)
                } else {
                    rules.insert(pos - 1, first_number)
                }
            } else {
                rules.push(first_number);
                rules.push(second_number);
            }
        }
    }
    rules
}

pub fn day_five_part_one() {
    let rules: Vec<i32> = load_rules();
    println!("{:?}", rules);
    let input: String = fs::read_to_string("./src/day_five/input.txt")
        .expect("Should have been able to read the file");
    for line in input.lines() {
        let strings: Vec<&str> = line.split(',').collect();
        let mut numbers: Vec<i32> = Vec::new();
        for string in strings {
            numbers.push(string.parse().unwrap());
        }
        let sorted_numbers: Vec<i32> = numbers.clone();
        //sorted_numbers.sort_by(|a, b| get_position(&rules, *a).cmp(get_position(&rules, b)));
        if numbers == sorted_numbers {}
    }
}
