// basic integration tests for the module

// define some example input structs
use letterboxed_solver::solvers::{a_star, brute_force, pre_dict};
use letterboxed_solver::NYTBoxPuzzle;

// todo: sure would be nice if I could have a helper function to test any algo...
// will have to wait until i go back and refactor the algos using the Strategy pattern & refer
// to them with a trait. (i think that's how I'd do it? still learning as i go.).
// for now i'm just gonna copypasta these around.

fn get_test_puzzles() -> Vec<NYTBoxPuzzle> {
    vec![
        NYTBoxPuzzle::from_str(6, "erb uln imk jav").unwrap(), // nov 6, 2024
    ]
}

#[test]
#[ignore] // because brute force takes soooooooo long
fn test_brute_force() {
    for puzzle in get_test_puzzles() {
        println!("TEST CASE: {}", puzzle);

        let solution = brute_force::solve_brute_force(&puzzle);

        assert!(solution.is_some());
        let solution = solution.unwrap();
        println!("{:?}", solution);
        assert!(puzzle.validate_solution(&solution).is_ok());
    }
}

#[test]
#[ignore] // still eats too much memory! run benchmark instead
fn test_pre_dict() {
    for puzzle in get_test_puzzles() {
        println!("TEST CASE: {}", puzzle);

        let solution = pre_dict::solve_pre_dict(&puzzle);

        assert!(solution.is_some());
        let solution = solution.unwrap();
        println!("{:?}", solution);
        assert!(puzzle.validate_solution(&solution).is_ok());
    }
}

#[test]
fn test_a_star() {
    for puzzle in get_test_puzzles() {
        println!("TEST CASE: {}", puzzle);

        let solution = a_star::solve_a_star(&puzzle);

        assert!(solution.is_some());
        let solution = solution.unwrap();
        println!("{:?}", solution);
        assert!(puzzle.validate_solution(&solution).is_ok());
    }
}
