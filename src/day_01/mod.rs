pub fn day_01_part_01_solve(data: Vec<&str>) -> i32 {
    let total_fuel_requirements: i32 = data
        .clone()
        .into_iter()
        .map(|module| -> i32 {
            let module_mass: i32 = module.parse().unwrap();
            return (((module_mass as f64) / 3.0).floor() - 2.0) as i32;
        })
        .sum();

    return total_fuel_requirements;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day_01_part_01_solve() {
        let mut data = Vec::new();
        data.push("12");
        assert_eq!(day_01_part_01_solve(data), 2);
    }
}
