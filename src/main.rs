use math;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let data: Vec<&str> = include_str!("./day-01/data.txt").trim().split_whitespace().collect();
    let mut sum_of_numbers = 0.0;
    for x in data.iter() {
        let number: f64 = x.parse().unwrap();
        let float_number: f64 = math::round::floor(number / 3.0, 0) - 2.0;
        sum_of_numbers += float_number;
    }
    println!("solution is: {}", sum_of_numbers);

    let mut vec: Vec<f64> = Vec::new();
    for data_element in data.iter() {
        let mut current_data =data_element.parse().unwrap();
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
