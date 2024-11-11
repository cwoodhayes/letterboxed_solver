use env_logger::Env;
use log::debug;
use std::env;

use letterboxed_solver::{
    solvers::{a_star, SolverStrategy},
    NYTBoxPuzzle,
};

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

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
    debug!("\nPUZZLE: {}", puzzle);

    // solve!
    let solution = match a_star::AStarSolver::solve(&puzzle) {
        Some(solution) => solution,
        None => {
            eprintln!("No solution found! :(");
            return;
        }
    };
    println!("\nPUZZLE: {}", puzzle);
    println!("SOLUTION: {:?}", solution);
}
