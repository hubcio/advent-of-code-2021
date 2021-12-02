use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn get_count(file_path: PathBuf) -> usize {
    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| line.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .map(|t| t[1] - t[0])
        .filter(|&val| val > 0)
        .count()
}

mod tests {
    use crate::day_1::a::get_count;
    use std::path::PathBuf;

    #[test]
    fn test_data() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_1/input_test.txt");

        assert_eq!(get_count(d.to_owned()), 7);
    }
}
