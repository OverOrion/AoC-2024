use std::{collections::BTreeMap, fs::read_to_string};

use crate::Solution;

#[derive(Debug, Default)]
pub struct Day01Solver {
    column_a: Vec<i32>,
    column_b: Vec<i32>,
}

impl Day01Solver {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Solution for Day01Solver {
    type SolutionResult = u64;

    fn get_input(&self) -> String {
        read_to_string("./src/inputs/day01.txt").expect("Failed to open file")
    }

    fn parse_input(&mut self, input: &str) {
        for line in input.lines() {
            let mut iter = line.split_whitespace();
            let (value_a, value_b) = (
                iter.next().expect("mailformed input"),
                iter.next().expect("mailformed input"),
            );
            let (value_a, value_b) = (
                value_a.parse::<i32>().expect("mailformed parsed input"),
                value_b.parse::<i32>().expect("mailformed parsed input"),
            );
            self.column_a.push(value_a);
            self.column_b.push(value_b);
        }
    }

    fn solve(mut self, input: &str) -> Vec<Self::SolutionResult> {
        self.parse_input(input);

        self.column_a.sort();
        self.column_b.sort();

        // Part 1
        let pairs = self.column_a.iter().zip(self.column_b.iter());
        let part1 = pairs.map(|(a, b)| a.abs_diff(*b) as u64).sum();

        // Part 2
        let mut column_b_values_with_occurences: BTreeMap<i32, i32> = BTreeMap::new();
        for value in self.column_b {
            *column_b_values_with_occurences.entry(value).or_insert(0) += 1;
        }
        let values_with_occurences = self.column_a.iter().map(|value_a| {
            let occurence = column_b_values_with_occurences
                .get(value_a)
                .unwrap_or(&0i32);
            (value_a, occurence)
        });
        let part2: u64 = values_with_occurences
            .map(|(value, occurence)| (value * occurence) as u64)
            .sum();
        vec![part1, part2]
    }

    fn print_solutions(self) {
        let input = self.get_input();
        for (ind, res) in self.solve(&input).iter().enumerate() {
            println!("Solution for part {} is: {}", ind + 1, res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let solver = Day01Solver::default();
        let example_input =
            read_to_string("./src/inputs/day01_example.txt").expect("Failed to open file");

        let example_solution = solver.solve(&example_input);

        let expected_result: u64 = 11;

        assert_eq!(example_solution[0], expected_result)
    }

    #[test]
    fn test_example_part2() {
        let solver = Day01Solver::default();
        let example_input =
            read_to_string("./src/inputs/day01_example.txt").expect("Failed to open file");

        let example_solution = solver.solve(&example_input);

        let expected_result: u64 = 31;

        assert_eq!(example_solution[1], expected_result)
    }
}
