use std::fs;
use std::path::PathBuf;

pub fn find_fish_amount(file_path: PathBuf, days: u32) -> usize {
    let mut fish = fs::read_to_string(file_path)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    for _ in 0..days {
        for f in 0..fish.len() {
            if fish[f] == 0 {
                fish[f] = 7;
                fish.push(8);
            }
            fish[f] -= 1;
        }
    }

    fish.len()
}

#[cfg(test)]
mod tests {
    use crate::day_6::a::find_fish_amount;
    use std::path::PathBuf;

    #[test]
    fn test_small() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_6/input_test.txt");
        assert_eq!(find_fish_amount(d.to_owned(), 18), 26);
        assert_eq!(find_fish_amount(d.to_owned(), 80), 5934);
    }

    #[test]
    fn test_real() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_6/input.txt");
        assert_eq!(find_fish_amount(d.to_owned(), 80), 366057);
    }
}
