use std::fs::read_to_string;

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

        let pairs = self.column_a.iter().zip(self.column_b.iter());
        let result = pairs.map(|(a, b)| a.abs_diff(*b) as u64).sum();
        vec![result]
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
    fn test_example() {
        let solver = Day01Solver::default();
        let example_input =
            read_to_string("./src/inputs/day01_example.txt").expect("Failed to open file");

        let example_solution = solver.solve(&example_input);

        let expected_result: u64 = 11;

        assert_eq!(example_solution[0], expected_result)
    }
}
