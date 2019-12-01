use math;

fn main() {
    let data = [113481,
    140620,
    123826,
    86474,
    71091,
    126880,
    103784,
    140154,
    124024,
    54281,
    80810,
    109441,
    68828,
    144207,
    99151,
    136876,
    99398,
    138555,
    118619,
    133215,
    139302,
    137780,
    136649,
    83358,
    63027,
    75067,
    73974,
    90158,
    94691,
    86847,
    61466,
    81184,
    86043,
    119923,
    116576,
    131380,
    102136,
    143364,
    124421,
    123141,
    138131,
    73274,
    84598,
    61410,
    67240,
    136186,
    63878,
    135804,
    73599,
    84526,
    116178,
    114587,
    58606,
    79162,
    124031,
    120329,
    61270,
    89887,
    54859,
    67618,
    96669,
    56796,
    55725,
    96105,
    68833,
    52417,
    72249,
    53930,
    139995,
    86217,
    131618,
    137145,
    54944,
    76456,
    82141,
    69754,
    102656,
    57461,
    108747,
    79510,
    105715,
    98046,
    116903,
    139339,
    127451,
    135374,
    88468,
    69524,
    76112,
    110928,
    99160,
    137229,
    121433,
    65951,
    56267,
    117209,
    61358,
    73659,
    69633,
    149274];

    let mut sum_of_numbers = 0.0;
    for x in data.iter() {
        let number: f64 = f64::from(*x);
        let float_number: f64 = math::round::floor(number / 3.0, 0) - 2.0;
        sum_of_numbers += float_number;
    }
    println!("solution is: {}", sum_of_numbers);

    let mut vec: Vec<f64> = Vec::new();
    for data_element in data.iter() {
        let mut current_data = f64::from(*data_element);
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
}
