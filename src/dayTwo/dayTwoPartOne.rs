fn main() {
    let input = fs::read_to_string("input.txt").expect("Should have been able to read the file");

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
