use std::fs;
use std::path::PathBuf;

fn median(numbers: &Vec<i32>) -> i32 {
    let mut numbers = numbers.clone();
    numbers.sort();
    numbers[numbers.len() / 2]
}

pub fn find_fuel_amount(file_path: PathBuf) -> i32 {
    let fuel = fs::read_to_string(file_path)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    println!("{:?}", fuel);

    let median = median(&fuel);

    let fuel_cost: i32 = fuel.iter().map(|f| i32::abs(f - median)).sum();

    println!("median={}, fuel_cost={}", median, fuel_cost);

    fuel_cost
}

#[cfg(test)]
mod tests {
    use crate::day_7::a::find_fuel_amount;
    use std::path::PathBuf;

    #[test]
    fn test_small() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_7/input_test.txt");
        assert_eq!(find_fuel_amount(d.to_owned()), 37);
    }

    #[test]
    fn test_real() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_7/input.txt");
        assert_eq!(find_fuel_amount(d.to_owned()), 344735);
    }
}
