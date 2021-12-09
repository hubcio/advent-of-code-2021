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
    done: bool,
    last_marked_element_value: u32,
}

impl Board {
    fn get_sum_of_non_marked_elements(&self) -> u32 {
        let mut sum = 0;
        for x in 0..5 {
            for y in 0..5 {
                if !self.rows[x][y].is_marked {
                    sum += self.rows[x][y].value;
                }
            }
        }
        sum
    }

    fn mark_number(&mut self, number: u32) -> Option<(usize, usize)> {
        let mut out: Option<_> = None;
        for x in 0..5 {
            for y in 0..5 {
                if self.rows[x][y].value == number {
                    self.rows[x][y].is_marked = true;
                    out = Some((x as usize, y as usize));
                    self.last_marked_element_value = number;
                }
            }
        }
        out
    }

    fn check_rows(&self, row_index: usize) -> bool {
        // check if whole row is marked
        let mut result = false;
        let mut numbers_marked = 0;
        for i in 0..5 {
            if self.rows[row_index][i].is_marked {
                numbers_marked += 1;
            }
            if numbers_marked == 5 {
                println!("WHOLE ROW MARKED! {:?}", self.rows[row_index][i]);
                result = true;
            }
        }
        result
    }

    fn check_columns(&self, column_index: usize) -> bool {
        let mut result = false;
        let mut numbers_marked = 0;
        for i in 0..5 {
            if self.rows[i][column_index].is_marked {
                numbers_marked += 1;
            }
            if numbers_marked == 5 {
                println!("WHOLE COLUMN MARKED!");
                result = true;
            }
        }
        result
    }
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
    let mut board = Board::default();

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
                board = Board::default();
            }
        }
    }
    let mut last_winning_board = Board::default();

    for number_to_mark in numbers_to_mark {
        for board in &mut boards {
            if !board.done {
                let result = board.mark_number(number_to_mark);

                if result.is_some() {
                    let result = result.unwrap();
                    if board.check_rows(result.0) {
                        board.done = true;
                        last_winning_board = *board;
                        continue;
                    }

                    if board.check_columns(result.1) {
                        board.done = true;
                        last_winning_board = *board;
                        continue;
                    }
                }
            }
        }
    }

    last_winning_board.get_sum_of_non_marked_elements()
        * last_winning_board.last_marked_element_value
}

#[cfg(test)]
mod tests {
    use crate::day_4::b::find_winner;
    use std::path::PathBuf;

    #[test]
    fn test_data() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("src/day_4/input_test.txt");
        assert_eq!(find_winner(d.to_owned()), 1924);
    }
}
