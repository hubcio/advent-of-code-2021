use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn validate(file_path: PathBuf) -> u32 {
    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut vec = Vec::new();
            let mut out = 0;
            for c in line.trim().chars() {
                match c {
                    ')' => {
                        if let Some('(') = vec.last() {
                            vec.pop();
                        } else {
                            out = 3;
                            break;
                        }
                    }
                    ']' => {
                        if let Some('[') = vec.last() {
                            vec.pop();
                        } else {
                            out = 57;
                            break;
                        }
                    }
                    '}' => {
                        if let Some('{') = vec.last() {
                            vec.pop();
                        } else {
                            out = 1197;
                            break;
                        }
                    }
                    '>' => {
                        if let Some('<') = vec.last() {
                            vec.pop();
                        } else {
                            out = 25137;
                            break;
                        }
                    }
                    _ => vec.push(c),
                }
            }
            out
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_10::a::validate;
    use std::path::PathBuf;

    #[test]
    fn test_small() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_10/input_test.txt");
        assert_eq!(validate(d.to_owned()), 26397);
    }

    #[test]
    fn test_real() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_10/input.txt");
        assert_eq!(validate(d.to_owned()), 299793);
    }
}
