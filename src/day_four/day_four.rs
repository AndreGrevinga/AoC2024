use std::fs;
use std::sync::Once;

static mut VEC_VEC: Vec<Vec<char>> = Vec::new();
static INIT: Once = Once::new();
fn get_cached_vec() -> Vec<Vec<char>> {
    unsafe {
        INIT.call_once(|| {
            let input = fs::read_to_string("./src/day_four/input.txt")
                .expect("Should have been able to read the file");
            for line in input.lines() {
                let letters: Vec<char> = line.chars().collect();
                VEC_VEC.push(letters);
            }
        });
        VEC_VEC.clone()
    }
}

pub fn day_four_part_one() {
    let mut result: i32 = 0;
    fn find_mas(start_x: i32, start_y: i32) -> i32 {
        let x_array: [i32; 3] = [-1, 0, 1];
        let y_array: [i32; 3] = [-1, 0, 1];
        let mut counter: i32 = 0;
        for y in y_array {
            for x in x_array {
                if !(x == 0 && y == 0) {
                    let new_x = start_x + x;
                    let new_y = start_y + y;
                    if find_char('M', new_x, new_y, x, y) {
                        counter += 1;
                    }
                }
            }
        }
        counter
    }
    for (y, vec) in get_cached_vec().iter().enumerate() {
        for (x, char) in vec.iter().enumerate() {
            if *char == 'X' {
                result += find_mas(x as i32, y as i32);
            }
        }
    }
    println!("{}", result)
}

fn find_char(char: char, start_x: i32, start_y: i32, change_x: i32, change_y: i32) -> bool {
    if has_char(char, start_x, start_y) {
        if char == 'S' {
            true
        } else {
            let next_char: char = match char {
                'M' => 'A',
                'A' => 'S',
                _ => 'z',
            };
            let new_x = start_x + change_x;
            let new_y = start_y + change_y;
            find_char(next_char, new_x, new_y, change_x, change_y)
        }
    } else {
        false
    }
}
fn has_char(char: char, x: i32, y: i32) -> bool {
    if y >= 0 || x >= 0 {
        let fixed_x = x as usize;
        let fixed_y = y as usize;
        let vector = get_cached_vec();
        let max_y = vector.len() - 1;
        if fixed_y > max_y {
            false
        } else {
            let inner_vector = &vector[fixed_y];
            let max_x = inner_vector.len() - 1;
            if fixed_x > max_x {
                false
            } else {
                inner_vector[fixed_x] == char
            }
        }
    } else {
        false
    }
}

pub fn day_four_part_two() {
    let mut result: i32 = 0;
    for (y, vec) in get_cached_vec().iter().enumerate() {
        for (x, char) in vec.iter().enumerate() {
            if *char == 'A' {
                if (find_char('M', x as i32 + 1, y as i32 - 1, -1, 1)
                    || find_char('M', x as i32 - 1, y as i32 + 1, 1, -1))
                    && (find_char('M', x as i32 + 1, y as i32 + 1, -1, -1)
                        || find_char('M', x as i32 - 1, y as i32 - 1, 1, 1))
                {
                    result += 1;
                }
            }
        }
    }
    println!("{}", result)
}
