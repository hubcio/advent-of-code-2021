use std::collections::HashSet;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn decode_digit(patterns: &[HashSet<char>], digit: &HashSet<char>) -> u32 {
    if digit.len() == patterns[0].len() {
        return 1;
    }
    if digit.len() == patterns[1].len() {
        return 7;
    }
    if digit.len() == patterns[2].len() {
        return 4;
    }
    if digit.len() == patterns[9].len() {
        return 8;
    }

    if digit.len() == 5 {
        if patterns[0].difference(digit).count() == 0 {
            return 3;
        } else if patterns[2].difference(digit).count() == 1 {
            return 5;
        } else {
            return 2;
        }
    } else {
        if patterns[0].difference(digit).count() > 0 {
            return 6;
        } else if patterns[2].difference(digit).count() > 0 {
            return 0;
        } else {
            return 9;
        }
    }
}

pub fn find_digits(file_path: PathBuf) -> u32 {
    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap().to_string();
            let (patterns_str, digits_str) = line.split_once(" | ").unwrap();

            let mut patterns_chars = patterns_str
                .split_whitespace()
                .map(|d| d.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();

            patterns_chars.sort_unstable_by_key(|p| p.len());

            let digits_chars: Vec<_> = digits_str
                .split_whitespace()
                .map(|d| d.chars().collect::<HashSet<_>>())
                .collect();

            let value = digits_chars
                .iter()
                .map(|digit| decode_digit(&patterns_chars, digit))
                .fold(0, |acc, d| acc * 10 + d);

            value
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_8::b::find_digits;
    use std::path::PathBuf;

    #[test]
    fn test_small() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_8/input_test.txt");
        assert_eq!(find_digits(d.to_owned()), 61229);
    }

    #[test]
    fn test_real() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_8/input.txt");
        assert_eq!(find_digits(d.to_owned()), 1084606);
    }
}
