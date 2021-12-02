use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        let value = parts[1].parse::<u32>().unwrap();

        Ok(match parts[0] {
            "forward" => Command::Forward(value),
            "down" => Command::Down(value),
            "up" => Command::Up(value),
            _ => panic!("Unknown command value"),
        })
    }
}

#[derive(Debug, PartialEq)]
struct Position {
    horizontal: u32,
    depth: u32,
    aim: u32,
}

impl Position {
    fn update(&mut self, command: Command) {
        match command {
            Command::Forward(v) => self.horizontal += v,
            Command::Down(v) => self.depth += v,
            Command::Up(v) => self.depth -= v,
        }
    }

    fn calculate(self) -> u32 {
        self.horizontal * self.depth
    }
}

pub fn get_position(file_path: PathBuf) -> u32 {
    let mut position = Position {
        horizontal: 0,
        depth: 0,
        aim: 0,
    };

    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| Command::from_str(line.as_ref().unwrap()).unwrap())
        .for_each(|c| position.update(c));

    position.calculate()
}

#[cfg(test)]
mod tests {
    use crate::day_2::a::get_position;
    use std::path::PathBuf;

    #[test]
    fn test_data() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_2/input_test.txt");
        assert_eq!(get_position(d.to_owned()), 150);
    }
}
