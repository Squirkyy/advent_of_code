use regex::Regex;
use std::fs;
use std::io::{self};

struct Number {
    start: usize,
    end: usize,
    num: i32,
    line: usize,
}

impl Number {
    fn has_adjacent_symbol(&self, results: &mut Vec<i32>, puzzle: &String) {
        let pattern = Regex::new("([*&/+%@#=$_-])").unwrap();
        let lines: Vec<&str> = puzzle.lines().collect();
        let puzzle_height = lines.len() as i32;

        for index in self.start..=self.end {
            let row = self.line as i32;
            let col = index as i32;

            for row_diff in -1..=1 {
                let line_index = row + row_diff;
                if line_index < 0 || line_index >= puzzle_height {
                    continue;
                }

                let line = lines[line_index as usize];
                for col_diff in -1..=1 {
                    let check_index = col + col_diff;
                    if check_index < 0
                        || check_index >= line.len() as i32
                        || (row_diff == 0 && col_diff == 0)
                    {
                        continue;
                    }

                    let test_char = line.chars().nth(check_index as usize).unwrap();
                    if pattern.is_match(&test_char.to_string()) {
                        results.push(self.num);
                        println!(
                            "Number {} at line {}, index {} has adjacent symbol",
                            self.num, self.line, index
                        );
                        return;
                    }
                }
            }
        }
    }
}

pub fn part1(path: &str) -> Result<i32, io::Error> {
    let mut results: Vec<i32> = Vec::new();
    let mut numbers: Vec<Vec<Number>> = Vec::new();
    let input = fs::read_to_string(path)?;
    for (i, line) in input.lines().enumerate() {
        numbers.push(find_number_in_string(line, i));
    }
    for ele in numbers {
        ele.iter()
            .for_each(|e| e.has_adjacent_symbol(&mut results, &input));
    }
    let result: i32 = results.iter().sum();
    Ok(result)
}

fn find_number_in_string(s: &str, line: usize) -> Vec<Number> {
    let mut result = Vec::new();
    let mut curr_number = String::new();
    let mut start_index = None;

    for (i, c) in s.chars().enumerate() {
        if c.is_digit(10) {
            curr_number.push(c);
            start_index = start_index.or(Some(i));
        } else if let Some(start) = start_index {
            if let Ok(num) = curr_number.parse::<i32>() {
                result.push(Number {
                    num,
                    start: start,
                    end: i - 1,
                    line: line,
                });
            }
            curr_number.clear();
            start_index = None;
        }
    }
    if let (Some(start), Ok(num)) = (start_index, curr_number.parse::<i32>()) {
        result.push(Number {
            num,
            start: start,
            end: s.len() - 1,
            line: line,
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::day3::part1;

    #[test]
    fn test() {
        let actual = match part1("src/day3/test.txt") {
            Ok(val) => val,
            Err(x) => panic!("{}", x),
        };
        assert_eq!(4361, actual);
    }
}
