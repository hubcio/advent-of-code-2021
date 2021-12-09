use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Copy, Clone, Default, Debug)]
struct Number {
    value: u32,
    is_marked: bool,
}

#[derive(Copy, Clone, Default, Debug)]
struct Board {
    rows: [[Number; 5]; 5],
    sum_of_non_marked_elements: u32,
}

pub fn find_winner(file_path: PathBuf) -> u32 {
    let mut lines = BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|l| l.unwrap());

    let numbers_to_mark: VecDeque<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let mut boards: Vec<Board> = vec![];

    let mut board = Board {
        rows: [[Number::default(); 5]; 5],
        sum_of_non_marked_elements: 0,
    };

    let mut current_line_index = 0;
    for line in &mut lines {
        if !line.is_empty() {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<u32>().unwrap())
                .enumerate()
                .for_each(|(i, v)| {
                    board.rows[current_line_index][i].value = v;
                    board.sum_of_non_marked_elements += v;
                });
            current_line_index += 1;
            if current_line_index == 5 {
                current_line_index = 0;
                boards.push(board);
                board = Board {
                    rows: [[Number::default(); 5]; 5],
                    sum_of_non_marked_elements: 0,
                };
            }
        }
    }

    for number_to_mark in numbers_to_mark {
        for board in &mut boards {
            for row in &mut board.rows {
                let position = row.iter().position(|&x| x.value == number_to_mark);
                if position.is_some() {
                    row[position.unwrap()].is_marked = true;
                    board.sum_of_non_marked_elements -= row[position.unwrap()].value;

                    // check if whole row is marked
                    let mut numbers_marked = 0;
                    for i in 0..5 {
                        if row[i].is_marked {
                            numbers_marked += 1;
                        }
                        if numbers_marked == 5 {
                            println!("WHOLE ROW MARKED! {:?}", row);
                            println!(
                                "board.sum_of_non_marked_elements={}, just_called={}",
                                board.sum_of_non_marked_elements,
                                row[position.unwrap()].value
                            );

                            return board.sum_of_non_marked_elements * row[position.unwrap()].value;
                        }
                    }
                }
            }
        }
    }
    1
}

#[cfg(test)]
mod tests {
    use crate::day_4::a::find_winner;
    use std::path::PathBuf;

    #[test]
    fn test_data() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_4/input_test.txt");
        assert_eq!(find_winner(d.to_owned()), 4512);
    }
}
