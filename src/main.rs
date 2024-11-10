use std::env;

use letterboxed_solver::{NYTBoxPuzzle, solvers::pre_dict};

fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() != 3 {
      eprintln!("Usage: letterboxed_solver [puzzle string] [max # of words]");
      return;
   }
   let puzz_str = args[1].clone();
   let max_words = args[2].parse::<usize>().unwrap();
   let puzzle = match NYTBoxPuzzle::from_str(max_words, &puzz_str) {
      Ok(puzzle) => puzzle,
      Err(e) => {
         eprintln!("Invalid puzzle! Error: {:?}", e);
         return;
      }
   };
   println!("\nPUZZLE: {}", puzzle);

   // solve!
   let solution = match pre_dict::solve_pre_dict(&puzzle) {
      Some(solution) => solution,
      None => {
         println!("No solution found! :(");
         return;
      }
   };
   println!("\nPUZZLE: {}", puzzle);
   println!("SOLUTION: {:?}", solution);
}