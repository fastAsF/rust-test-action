pub fn day_02_solve_part_01(data: Vec<i64>) {
    let mut counter = 0;
    let mut data_clone = data.clone();
    for x in 0..data_clone.len() {
        counter = counter + 1;
        if counter == 4 {
            counter = 0;
            if x == 1 {
                data_clone[x + 3] = data_clone[x + 1] + data_clone[x + 2];
            } else if x == 2 {
                data_clone[x + 3] = data_clone[x + 1] * data_clone[x + 2];
            }
        }
    }
    println!("The data after some function: {:?}", data_clone);
}
