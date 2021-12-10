use itertools::Itertools;
use pathfinding::prelude::{Matrix, Weights};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn print_matrix(m: &Matrix<u32>) {
    for k in 0..m.rows {
        for n in 0..m.columns() {
            print!("{}", m[&(k, n)]);
        }
        println!("");
    }
}

fn parse_input(file_path: PathBuf) -> Matrix<u32> {
    let lines = BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut m = Matrix::new_empty(lines[0].len());

    for line in lines {
        let x: Vec<u32> = line.chars().map(|f| f.to_digit(10).unwrap()).collect();
        m.extend(&x).unwrap();
    }
    m
}

pub fn find_risk(file_path: PathBuf) -> u32 {
    let m = parse_input(file_path);
    print_matrix(&m);

    let product = (0..m.rows).cartesian_product(0..m.columns);

    product
        .filter_map(|(r, c)| {
            let v = m[&(r, c)];
            m.neighbours(&(r, c), false)
                .all(|n| m[&n] > v)
                .then(|| v + 1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_9::a::find_risk;
    use std::path::PathBuf;

    #[test]
    fn test_small() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_9/input_test.txt");
        assert_eq!(find_risk(d.to_owned()), 15);
    }

    #[test]
    fn test_real() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_9/input.txt");
        assert_eq!(find_risk(d.to_owned()), 508);
    }
}
