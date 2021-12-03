use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
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

impl Measurement {
    fn get_common_or_uncommon_item_by_index(
        data: &Vec<Measurement>,
        index: usize,
        value_of_desired_item: u32,
        favor_ones: bool,
    ) -> u32 {
        let number_of_ones = data
            .iter()
            .filter(|x| x.data[index] == value_of_desired_item)
            .collect::<Vec<&Measurement>>()
            .len();
        let number_of_zeros = data.len() - number_of_ones;
        if number_of_ones == number_of_zeros {
            if favor_ones {
                return 1;
            } else {
                return 0;
            }
        }
        if number_of_ones > number_of_zeros {
            return 1;
        }

        if number_of_ones < number_of_zeros {
            return 0;
        }
        panic!("should not reach here ;)")
    }
}

#[derive(Debug, Clone)]
struct LifeSupportRatingCalculator {
    measurements: Vec<Measurement>,
}

impl LifeSupportRatingCalculator {
    fn insert_measurement(&mut self, m: Measurement) {
        self.measurements.push(m);
    }

    // some recurrence shit
    fn calculate_rating(
        data: &Vec<Measurement>,
        index: usize,
        value_of_desired_item: u32,
        favor_ones: bool,
    ) -> Vec<Measurement> {
        if data.len() == 1 {
            return data.clone();
        }
        let most_common_item = Measurement::get_common_or_uncommon_item_by_index(
            &data,
            index,
            value_of_desired_item,
            favor_ones,
        );
        let mut out: Vec<Measurement> = vec![];
        for d in data.iter() {
            if d.data[index] == most_common_item {
                out.push(d.clone());
            }
        }
        LifeSupportRatingCalculator::calculate_rating(
            &out,
            index + 1,
            value_of_desired_item,
            favor_ones,
        )
    }

    fn get_oxygen_generator_measurement(&mut self) -> Vec<Measurement> {
        LifeSupportRatingCalculator::calculate_rating(&self.measurements, 0, 1, true)
    }

    fn get_co2_scrubber_measurement(&mut self) -> Vec<Measurement> {
        LifeSupportRatingCalculator::calculate_rating(&self.measurements, 0, 0, false)
    }

    fn rating_to_decimal(measurement: Measurement) -> u32 {
        measurement
            .data
            .iter()
            .rev()
            .enumerate()
            .map(|(index, value)| value * u32::pow(2, index as u32))
            .sum()
    }

    fn calculate_total_rating(&mut self) -> u32 {
        let oxygen = self.get_oxygen_generator_measurement();
        let co2 = self.get_co2_scrubber_measurement();

        LifeSupportRatingCalculator::rating_to_decimal(oxygen[0].clone())
            * LifeSupportRatingCalculator::rating_to_decimal(co2[0].clone())
    }
}

pub fn get_life_support_rating(file_path: PathBuf) -> u32 {
    let mut calculator = LifeSupportRatingCalculator {
        measurements: vec![],
    };

    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| Measurement::from_str(line.as_ref().unwrap()).unwrap())
        .for_each(|c| calculator.insert_measurement(c));

    calculator.calculate_total_rating()
}

#[cfg(test)]
mod tests {
    use crate::day_3::b::get_life_support_rating;
    use std::path::PathBuf;

    #[test]
    fn test_data() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_3/input_test.txt");
        assert_eq!(get_life_support_rating(d.to_owned()), 230);
    }
}
