use std::fs;

fn prepare_numbers(input: &String) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    for ch in input.chars() {
        if let Some(digit) = ch.to_digit(10) {
            numbers.push(digit as i32);
        }
    }
    let mut counter: i32 = 0;
    let mut second_numbers: Vec<i32> = Vec::new();
    for (index, num) in numbers.iter().enumerate() {
        let mut number: i32 = 0;
        if index % 2 == 0 {
            number = counter;
            counter += 1;
        } else {
            number = -1;
        };
        for _ in 0..*num {
            second_numbers.push(number);
        }
    }
    second_numbers
}
fn calculate_checksum(numbers: Vec<i32>) -> i128 {
    let mut sum: i128 = 0;
    for (index, num) in numbers.iter().enumerate() {
        if !(*num == -1) {
            sum += (index as i32 * num) as i128;
        }
    }
    sum
}

fn find_free_space(vec: &Vec<i32>, block_size: i32, max_index: usize) -> i32 {
    let mut index: i32 = 0;
    let mut free_space: i32 = 0;
    while index < max_index as i32 {
        if vec[index as usize] == -1 {
            free_space += 1;
        } else {
            free_space = 0;
        }
        if free_space == block_size {
            return index - block_size + 1;
        }
        index += 1;
    }
    -1
}

pub fn day_nine_part_one() {
    let input: String = fs::read_to_string("./src/day_nine/input.txt")
        .expect("Should have been able to read the file");
    let numbers = prepare_numbers(&input);
    let mut free_space_index: usize = 0 as usize;
    let mut second_numbers = numbers.clone();
    let numbers_len: usize = numbers.len();
    let mut counter = 0;
    let mut previous_number = 0;
    for (reverse_index, num) in numbers.into_iter().rev().enumerate() {
        if num == previous_number {
            counter += 1;
            continue;
        }
        let mut is_sorted: bool = true;
        let mut previous_el = 0;
        for el in &second_numbers {
            if !(*el == -1) && previous_el == -1 {
                is_sorted = false;
            }
            previous_el = *el;
        }
        if is_sorted {
            break;
        }
        while second_numbers.len() > free_space_index && !(second_numbers[free_space_index] == -1) {
            free_space_index += 1;
        }
        let num_index = (numbers_len - reverse_index) - 1;
        second_numbers[free_space_index] = num;
        second_numbers.remove(num_index);
        previous_number = num;
    }
    println!("{}", calculate_checksum(second_numbers));
}

pub fn day_nine_part_two() {
    let input: String = fs::read_to_string("./src/day_nine/input.txt")
        .expect("Should have been able to read the file");
    let numbers = prepare_numbers(&input);
    let mut second_numbers = numbers.clone();
    let numbers_len: usize = numbers.len();
    let mut previous_number = -1;
    let mut block_size = 1;
    for (reverse_index, num) in numbers.into_iter().rev().enumerate() {
        if num == previous_number {
            block_size += 1;
        } else {
            if previous_number != -1 {
                let num_index: usize = numbers_len - reverse_index; //no -1 because we want the index of the previous number
                let starting_index = find_free_space(&second_numbers, block_size, num_index);
                if starting_index != -1 {
                    for index in 0..block_size {
                        second_numbers[(starting_index + index) as usize] = previous_number;
                        second_numbers[num_index + index as usize] = -1;
                    }
                }
            }
            previous_number = num;
            block_size = 1;
        }
    }
    //println!("{:?}", second_numbers);
    println!("{}", calculate_checksum(second_numbers));
}
