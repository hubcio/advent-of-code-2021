use core::num;
use std::fs;
use std::path::PathBuf;

fn steps_sum(number: i32) -> i32 {
    ((1 + number) * number) / 2
}

fn calculate_cost(numbers: &Vec<i32>, desired_position: i32) -> i32 {
    let out = numbers
        .iter()
        .map(|f| steps_sum(i32::abs(f - desired_position)))
        .sum();
    out
}

pub fn find_fuel_amount(file_path: PathBuf) -> i32 {
    let fuel: Vec<i32> = fs::read_to_string(file_path)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let min = *fuel.iter().min().unwrap();
    let max = *fuel.iter().max().unwrap();

    let mut out = i32::MAX;
    for pos in min..=max {
        let x = calculate_cost(&fuel, pos);
        if x < out {
            out = x;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use crate::day_7::b::find_fuel_amount;
    use std::path::PathBuf;

    #[test]
    fn test_small() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_7/input_test.txt");
        assert_eq!(find_fuel_amount(d.to_owned()), 168);
    }

    #[test]
    fn test_real() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_7/input.txt");
        assert_eq!(find_fuel_amount(d.to_owned()), 96798233);
    }
}
