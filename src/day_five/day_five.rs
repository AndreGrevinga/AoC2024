use std::fs;

fn get_position(vec: &Vec<i32>, number: i32) -> usize {
    vec.iter().position(|&num| num == number).unwrap()
}

fn insert_at_index(vec: &mut Vec<i32>, pos: usize, num: i32) {
    if pos >= vec.len() {
        vec.push(num);
    } else {
        vec.insert(pos, num);
    }
}

fn load_rules() -> Vec<i32> {
    let mut rules: Vec<i32> = Vec::new();
    //let rule_file: String = fs::read_to_string("./src/day_five/rules.txt")
    //    .expect("Should have been able to read the file");
    let rule_file = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13";
    for line in rule_file.lines() {
        let numbers: Vec<&str> = line.split('|').collect();
        let first_number: i32 = numbers[0].parse().unwrap();
        let second_number: i32 = numbers[1].parse().unwrap();
        if rules.contains(&first_number) {
            if rules.contains(&second_number) {
                let pos_one: usize = get_position(&rules, first_number);
                let pos_two: usize = get_position(&rules, second_number);
                if pos_one > pos_two {
                    let mut copy = rules.clone();
                    let (first_part, second_part) = copy.split_at_mut(pos_two + 1);
                    rules = second_part.to_vec();
                    rules.append(&mut first_part.to_vec());
                }
            } else {
                let pos: usize = get_position(&rules, first_number);
                insert_at_index(&mut rules, pos + 1, second_number);
            }
        } else {
            if rules.contains(&second_number) {
                let pos: usize = get_position(&rules, second_number);
                rules.insert(pos, first_number)
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
    //let input: String = fs::read_to_string("./src/day_five/input.txt")
    //    .expect("Should have been able to read the file");
    let input = "75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    let mut sum: i32 = 0;
    for line in input.lines() {
        let strings: Vec<&str> = line.split(',').collect();
        let mut numbers: Vec<i32> = Vec::new();

        for string in strings {
            numbers.push(string.parse().unwrap());
        }
        let mut sorted_numbers: Vec<i32> = numbers.clone();
        sorted_numbers.sort_by(|a, b| get_position(&rules, *a).cmp(&get_position(&rules, *b)));
        //println!("{:?} | {:?}", numbers, sorted_numbers);
        if numbers == sorted_numbers {
            let index: usize = (numbers.len() / 2) + 1;
            println!("{}", index);
            sum += numbers[index];
        }
    }
    println!("{}", sum)
}
