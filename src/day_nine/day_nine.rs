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

fn number_size(vec: &Vec<i32>, num: i32, start_index: usize) -> i32 {
    let mut size: i32 = 0;
    let mut index: usize = start_index;
    let len: usize = vec.len();
    while index < len && vec[index] == num {
        size += 1;
        index += 1;
    }
    size
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
    //println!("{:?}", second_numbers);
    println!("{}", calculate_checksum(second_numbers));
}

pub fn day_nine_part_two() {
    let input: String = fs::read_to_string("./src/day_nine/input.txt")
        .expect("Should have been able to read the file");
    let numbers = prepare_numbers(&input);
    let mut second_numbers = numbers.clone();
    let numbers_len: usize = numbers.len();
    for (reverse_index, num) in numbers.into_iter().rev().enumerate() {
        if !(num == -1) {
            while second_numbers.len() > free_space_index
                && !(second_numbers[free_space_index] == -1)
            {
                free_space_index += 1;
            }
            let num_index = (numbers_len - reverse_index) - 1;
            second_numbers[free_space_index] = num;
            second_numbers.remove(num_index);
        }
    }
    //println!("{:?}", second_numbers);
    println!("{}", calculate_checksum(second_numbers));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_size() {
        let vec = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(number_size(&vec, 1, 0), 3);
        assert_eq!(number_size(&vec, 2, 3), 2);
        assert_eq!(number_size(&vec, 3, 5), 1);
    }

    #[test]
    fn test_number_size_empty() {
        let vec: Vec<i32> = vec![];
        assert_eq!(number_size(&vec, 1, 0), 0);
    }

    #[test]
    fn test_number_size_single() {
        let vec = vec![5];
        assert_eq!(number_size(&vec, 5, 0), 1);
    }
}
