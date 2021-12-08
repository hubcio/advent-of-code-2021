use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn find_digits_1_4_7_8(file_path: PathBuf) -> usize {
    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .split(" | ")
                .nth(1)
                .unwrap()
                .to_string()
                .split(' ')
                .filter(|x| x.len() == 2 || x.len() == 3 || x.len() == 4 || x.len() == 7)
                .count()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use crate::day_8::a::find_digits_1_4_7_8;
    use std::path::PathBuf;

    #[test]
    fn test_small() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_8/input_test.txt");
        assert_eq!(find_digits_1_4_7_8(d.to_owned()), 26);
    }

    #[test]
    fn test_real() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_8/input.txt");
        assert_eq!(find_digits_1_4_7_8(d.to_owned()), 539);
    }
}
