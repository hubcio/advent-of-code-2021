use std::fmt::{self};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Default, Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Default, Debug, PartialEq)]
struct Vector {
    start: Point,
    end: Point,
}

impl FromStr for Vector {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vectors_str: Vec<Vec<&str>> = s.split(" -> ").map(|x| x.split(",").collect()).collect();

        let x1 = vectors_str[0][0].parse::<usize>().unwrap();
        let y1 = vectors_str[0][1].parse::<usize>().unwrap();
        let x2 = vectors_str[1][0].parse::<usize>().unwrap();
        let y2 = vectors_str[1][1].parse::<usize>().unwrap();

        let p1 = Point { x: x1, y: y1 };
        let p2 = Point { x: x2, y: y2 };

        Ok(Vector { start: p1, end: p2 })
    }
}

struct Diagram {
    counts: Vec<Vec<usize>>,
}

impl Diagram {
    fn count_points_from_vector(&mut self, vector: Vector) {
        if vector.start.x == vector.end.x {
            // horizontal vector increasing
            for i in vector.start.y..=vector.end.y {
                self.counts[i][vector.end.x] += 1;
            }
            // horizonal vector decreasing
            for i in vector.end.y..=vector.start.y {
                self.counts[i][vector.end.x] += 1;
            }
        } else if vector.start.y == vector.end.y {
            // vertical vector increasing
            for i in vector.start.x..=vector.end.x {
                self.counts[vector.end.y][i] += 1;
            }
            // vertical vector decreasing
            for i in vector.end.x..=vector.start.x {
                self.counts[vector.end.y][i] += 1;
            }
        } else if vector.start.x > vector.end.x && vector.start.y > vector.end.y {
            let mut starting_point = Point {
                x: vector.start.x,
                y: vector.start.y,
            };

            loop {
                self.counts[starting_point.y][starting_point.x] += 1;
                starting_point.x -= 1;
                starting_point.y -= 1;
                if starting_point == vector.end {
                    self.counts[starting_point.y][starting_point.x] += 1;
                    break;
                }
            }
        } else if vector.start.x < vector.end.x && vector.start.y < vector.end.y {
            let mut starting_point = Point {
                x: vector.start.x,
                y: vector.start.y,
            };

            loop {
                self.counts[starting_point.y][starting_point.x] += 1;
                starting_point.x += 1;
                starting_point.y += 1;
                if starting_point == vector.end {
                    self.counts[starting_point.y][starting_point.x] += 1;
                    break;
                }
            }
        } else if vector.start.x > vector.end.x && vector.start.y < vector.end.y {
            let mut starting_point = Point {
                x: vector.start.x,
                y: vector.start.y,
            };

            loop {
                self.counts[starting_point.y][starting_point.x] += 1;
                starting_point.x -= 1;
                starting_point.y += 1;
                if starting_point == vector.end {
                    self.counts[starting_point.y][starting_point.x] += 1;
                    break;
                }
            }
        } else if vector.start.x < vector.end.x && vector.start.y > vector.end.y {
            let mut starting_point = Point {
                x: vector.start.x,
                y: vector.start.y,
            };

            loop {
                self.counts[starting_point.y][starting_point.x] += 1;
                starting_point.x += 1;
                starting_point.y -= 1;
                if starting_point == vector.end {
                    self.counts[starting_point.y][starting_point.x] += 1;
                    break;
                }
            }
        }
    }

    fn get_number_of_dangerous_areas(self) -> usize {
        let mut acc: usize = 0;
        self.counts.into_iter().for_each(|rows| {
            rows.into_iter().for_each(|count| {
                if count > 1 {
                    acc += 1;
                }
            });
        });
        acc
    }
}

impl fmt::Display for Diagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(self.counts.iter().for_each(|rows| {
            rows.into_iter().for_each(|count| {
                if *count > 0 {
                    write!(f, "{}", count).unwrap();
                } else {
                    write!(f, ".").unwrap();
                }
            });
            writeln!(f).unwrap();
        }))
    }
}

pub fn find_overlapping_points(file_path: PathBuf) -> u32 {
    let mut diagram = Diagram {
        counts: vec![vec![0; 10]; 10],
    };

    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| Vector::from_str(line.as_ref().unwrap()).unwrap())
        .for_each(|v| diagram.count_points_from_vector(v));

    // println!("{}", diagram);

    diagram.get_number_of_dangerous_areas() as u32
}

#[cfg(test)]
mod tests {
    use crate::day_5::b::find_overlapping_points;
    use std::path::PathBuf;

    #[test]
    fn test_data() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_5/input_test.txt");
        assert_eq!(find_overlapping_points(d.to_owned()), 12);
    }
}
