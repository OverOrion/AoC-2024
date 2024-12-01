pub trait Solution {
    type SolutionResult;
    fn get_input(&self) -> String;
    fn parse_input(&mut self, input: &str);
    fn solve(self, input: &str) -> Vec<Self::SolutionResult>;
    fn print_solutions(self);
}

pub mod solutions;
