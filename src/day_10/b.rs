use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

pub fn validate(file_path: PathBuf) -> u64 {
    let mut scores: Vec<_> = BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut vec = Vec::new();
            let mut corrupted = false;
            for c in line.trim().chars() {
                match c {
                    ')' => {
                        if let Some('(') = vec.last() {
                            vec.pop();
                        } else {
                            corrupted = true;
                            break;
                        }
                    }
                    ']' => {
                        if let Some('[') = vec.last() {
                            vec.pop();
                        } else {
                            corrupted = true;
                            break;
                        }
                    }
                    '}' => {
                        if let Some('{') = vec.last() {
                            vec.pop();
                        } else {
                            corrupted = true;
                            break;
                        }
                    }
                    '>' => {
                        if let Some('<') = vec.last() {
                            vec.pop();
                        } else {
                            corrupted = true;
                            break;
                        }
                    }
                    _ => vec.push(c),
                }
            }

            if corrupted {
                return 0;
            }

            let mut points = 0;
            for c in vec.into_iter().rev() {
                points *= 5;
                match c {
                    '(' => points += 1,
                    '[' => points += 2,
                    '{' => points += 3,
                    '<' => points += 4,
                    _ => panic!(),
                }
            }
            points
        })
        .filter(|&score| score != 0)
        .collect();

    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use crate::day_10::b::validate;
    use std::path::PathBuf;

    #[test]
    fn test_small() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_10/input_test.txt");
        assert_eq!(validate(d.to_owned()), 288957);
    }

    #[test]
    fn test_real() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_10/input.txt");
        assert_eq!(validate(d.to_owned()), 3654963618);
    }
}
