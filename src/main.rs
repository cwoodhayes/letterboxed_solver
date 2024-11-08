use std::env;

use letterboxed_solver::{NYTBoxPuzzle, solvers::pre_dict};

fn main() {
   let args: Vec<String> = env::args().collect();
   let puzz_str = args[1].clone();
   let max_words = args[2].parse::<usize>().unwrap();
   let puzzle = match NYTBoxPuzzle::from_str(max_words, &puzz_str) {
      Ok(puzzle) => puzzle,
      Err(e) => {
         println!("Invalid puzzle! Error: {:?}", e);
         return;
      }
   };
   
   // solve!
   let solution = match pre_dict::solve_pre_dict(&puzzle) {
      Some(solution) => solution,
      None => {
         println!("\nPUZZLE: {:?}", puzz_str);
         println!("No solution found! :(");
         return;
      }
   };
   println!("\nPUZZLE: {} words for {:?}", max_words, puzz_str);
   println!("SOLUTION: {:?}", solution);
}