use std::fs;

pub fn day_nine_part_one() {
    let input: String = fs::read_to_string("./src/day_nine/input.txt")
        .expect("Should have been able to read the file");
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
    let mut free_space_index: usize = 0 as usize;
    let mut third_numbers = second_numbers.clone();
    let numbers_len: usize = second_numbers.len();
    for (reverse_index, num) in second_numbers.into_iter().rev().enumerate() {
        if !(num == -1) {
            let mut is_sorted: bool = true;
            let mut previous_el = 0;
            for el in &third_numbers {
                if !(*el == -1) && previous_el == -1 {
                    is_sorted = false;
                }
                previous_el = *el;
            }
            if is_sorted {
                break;
            }
            while third_numbers.len() > free_space_index && !(third_numbers[free_space_index] == -1)
            {
                free_space_index += 1;
            }
            let num_index = (numbers_len - reverse_index) - 1;
            third_numbers[free_space_index] = num;
            third_numbers.remove(num_index);
        }
    }
    let mut sum: i128 = 0;
    for (index, num) in third_numbers.iter().enumerate() {
        if !(*num == -1) {
            sum += (index as i32 * num) as i128;
        }
    }
    println!("{:?}", third_numbers);
    println!("{}", sum);
}
