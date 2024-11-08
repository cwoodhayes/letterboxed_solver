use std::env;

use letterboxed_solver::{NYTBoxPuzzle, solvers::brute_force, solvers::pre_dict};

fn main() {
   let args: Vec<String> = env::args().collect();
   dbg!(args);

   let nov_6_2024 = NYTBoxPuzzle::from_str(6, "erb uln imk jav").unwrap();
   println!("{:?}", nov_6_2024);

   let solution = pre_dict::solve_pre_dict(&nov_6_2024);
   assert!(solution.is_some());
   let solution = solution.unwrap();
   assert!(nov_6_2024.validate_solution(&solution).is_ok())
}