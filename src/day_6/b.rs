use std::collections::VecDeque;
use std::fmt::{self};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

type FishCounter = [usize; 9];

pub fn find_fish_amount(file_path: PathBuf, days: usize) -> usize {
    // store fish in helper array counter
    // index of array is also fish days to breed
    // [3,4,3,1,2] will map to [0,1,1,2,1,0,0,0,0]
    let mut f: FishCounter = [0; 9];

    fs::read_to_string(file_path)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
        .iter()
        .for_each(|&u| f[u] += 1);

    for _ in 0..days {
        let zero_day_fish = f[0];
        f[0] = f[1];
        f[1] = f[2];
        f[2] = f[3];
        f[3] = f[4];
        f[4] = f[5];
        f[5] = f[6];
        f[6] = f[7] + zero_day_fish;
        f[7] = f[8];
        f[8] = zero_day_fish;
    }
    f.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::day_6::b::find_fish_amount;
    use std::path::PathBuf;

    #[test]
    fn test_small() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_6/input_test.txt");
        assert_eq!(find_fish_amount(d.to_owned(), 18), 26);
        assert_eq!(find_fish_amount(d.to_owned(), 80), 5934);
        assert_eq!(find_fish_amount(d.to_owned(), 256), 26984457539);
    }

    #[test]
    fn test_real() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_6/input.txt");
        assert_eq!(find_fish_amount(d.to_owned(), 80), 366057);
        assert_eq!(find_fish_amount(d.to_owned(), 256), 1653559299811);
    }
}
