use math;
use std::time::Instant;

mod day_01;
mod day_02;

fn main() {
    let now = Instant::now();
    let data: Vec<&str> = include_str!("./day_01/data.txt")
        .trim()
        .split_whitespace()
        .collect();
    let data_day02: Vec<i64> = include_str!("./day_02/data.txt")
        .trim()
        .split(",")
        .map(|x| -> i64 { x.parse().unwrap() })
        .collect();
    println!("{:?}", data_day02);
    let sum_of_numbers: i32 = day_01::day_01_part_01_solve(data.clone());
    println!("Function solve: {:?}", sum_of_numbers);

    day_02::day_02_solve_part_01(data_day02);
    let mut vec: Vec<f64> = Vec::new();
    for data_element in data.iter() {
        let mut current_data = data_element.parse().unwrap();
        while current_data != 0.0 {
            current_data = math::round::floor(current_data / 3.0, 0);
            if current_data != 0.0 {
                current_data = current_data - 2.0;
                if current_data > 0.0 {
                    vec.push(current_data);
                } else {
                    current_data = 0.0;
                }
            }
        }
    }

    let mut solution: f64 = 0.0;
    for element in vec.iter() {
        solution += element;
    }
    println!("Solution part 2: {:?}", solution);
    println!("We got the solution in: {:?}", now.elapsed());
}
