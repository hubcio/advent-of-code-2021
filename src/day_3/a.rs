use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
struct Measurement {
    data: Vec<u32>,
}

impl FromStr for Measurement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Measurement {
            data: s
                .chars()
                .map(|x| match x {
                    '0' => 0b0,
                    '1' => 0b1,
                    _ => panic!("Invalid character value!"),
                })
                .collect::<Vec<u32>>(),
        })
    }
}

#[derive(Debug)]
struct Frequency {
    bits_frequency: Vec<u32>,
    number_of_measurements: u32,
}

impl Frequency {
    fn update(&mut self, m: Measurement) {
        while self.bits_frequency.len() != m.data.len() {
            self.bits_frequency.push(0);
        }

        m.data.iter().enumerate().for_each(|(index, bit)| {
            if *bit == 0b1 {
                self.bits_frequency[index] += 1;
            }
        });
        self.number_of_measurements += 1;
    }

    fn get_gamma_rate(&self) -> u32 {
        self.bits_frequency
            .iter()
            .rev()
            .enumerate()
            .map(|(index, value)| {
                if value > &(self.number_of_measurements / 2) {
                    0b1 * u32::pow(2, index as u32)
                } else {
                    0b0
                }
            })
            .sum()
    }

    fn get_epsilon_rate(&self) -> u32 {
        self.bits_frequency
            .iter()
            .rev()
            .enumerate()
            .map(|(index, value)| {
                if value < &(self.number_of_measurements / 2) {
                    0b1 * u32::pow(2, index as u32)
                } else {
                    0b0
                }
            })
            .sum()
    }
}

pub fn get_rate(file_path: PathBuf) -> u32 {
    let mut rates = Frequency {
        bits_frequency: vec![],
        number_of_measurements: 0,
    };

    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| Measurement::from_str(line.as_ref().unwrap()).unwrap())
        .for_each(|c| rates.update(c));

    rates.get_gamma_rate() * rates.get_epsilon_rate()
}

#[cfg(test)]
mod tests {
    use crate::day_3::a::get_rate;
    use std::path::PathBuf;

    #[test]
    fn test_data() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_3/input_test.txt");
        assert_eq!(get_rate(d.to_owned()), 198);
    }
}
