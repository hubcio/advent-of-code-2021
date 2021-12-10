use itertools::Itertools;
use pathfinding::prelude::Matrix;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

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

pub fn find_risk(file_path: PathBuf) -> usize {
    let matrix = parse_input(file_path);

    let mut basins = HashMap::<(usize, usize), usize>::new();

    (0..matrix.rows)
        .cartesian_product(0..matrix.columns)
        .for_each(|point| {
            let mut point = point;
            let mut value = matrix[&point];

            while let Some(n) = (value != 9)
                .then(|| matrix.neighbours(&point, false).find(|n| matrix[n] < value))
                .flatten()
            {
                point = n;
                value = matrix[&point];
            }

            *basins.entry(point).or_insert(0) += 1;
        });

    basins.values().sorted().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use crate::day_9::b::find_risk;
    use std::path::PathBuf;

    #[test]
    fn test_small() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_9/input_test.txt");
        assert_eq!(find_risk(d.to_owned()), 1134);
    }

    #[test]
    fn test_real() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_9/input.txt");
        assert_eq!(find_risk(d.to_owned()), 1564640);
    }
}
