use rusttype::Point;
use std::fs;

fn count_neighbours(
    input: &Vec<Vec<char>>,
    point: Point<usize>,
    char: char,
    mut tuple: (Vec<Point<usize>>, i32),
) -> (Vec<Point<usize>>, i32) {
    let x: usize = point.x;
    let y: usize = point.y;
    tuple.0.push(point);
    println!("{:?}", point);
    if x == 0 {
        tuple.1 += 1;
    } else {
        if input[point.y][x - 1] == char {
            let new_point = Point { x: x - 1, y };
            if !tuple.0.contains(&new_point) {
                tuple = count_neighbours(input, new_point, char, tuple.clone());
            }
        } else {
            tuple.1 += 1;
        }
    }
    if y == 0 {
        tuple.1 += 1;
    } else {
        if input[y - 1][x] == char {
            let new_point = Point { x, y: y - 1 };
            if !tuple.0.contains(&new_point) {
                tuple = count_neighbours(input, new_point, char, tuple.clone());
            }
        } else {
            tuple.1 += 1;
        }
    }
    if x == input[y].len() - 1 {
        tuple.1 += 1;
    } else {
        if input[y][x + 1] == char {
            let new_point: Point<usize> = Point { x: x + 1, y };
            if !tuple.0.contains(&new_point) {
                tuple = count_neighbours(input, new_point, char, tuple.clone());
            }
        } else {
            tuple.1 += 1;
        }
    }
    if y == input.len() - 1 {
        tuple.1 += 1;
    } else {
        if input[y + 1][x] == char {
            let new_point = Point { x, y: y + 1 };
            if !tuple.0.contains(&new_point) {
                tuple = count_neighbours(input, new_point, char, tuple.clone());
            }
        } else {
            tuple.1 += 1;
        }
    }
    tuple
}

pub fn part_one() {
    let input = fs::read_to_string("./src/day_twelve/input.txt")
        .expect("Should have been able to read the file");
    let mut vec: Vec<Vec<char>> = input
        .lines()
        .map(|str| str.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let x_len = vec[0].len();
    let mut sum: i64 = 0;
    for y in 0..vec.len() {
        for x in 0..x_len {
            let point = Point { x, y };
            println!("{:?}", point);
            let char = vec[y][x];
            if !(char == ' ') {
                let result: (Vec<Point<usize>>, i32) =
                    count_neighbours(&vec, point, char, (vec![], 0));
                let len = result.0.len() as i32;
                for point in result.0 {
                    vec[point.y][point.x] = ' ';
                }
                println!("{}", len * result.1);
                sum += (len * result.1) as i64;
            }
        }
    }
    println!("{}", sum)
}
