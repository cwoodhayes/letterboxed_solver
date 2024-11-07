// basic integration tests for the module

// define some example input structs
mod nyt_examples {
    use letterboxed_solver::solver::{brute_force, };
    use letterboxed_solver::NYTBoxPuzzle;

    #[test]
    fn test_example() {
        let nov_6_2024 = NYTBoxPuzzle::from_str(6, "erb uln imk jav");
        let nov_6_2024 = nov_6_2024.unwrap();
        
        println!("{:?}", nov_6_2024);

        let solution = brute_force::solve_brute_force(&nov_6_2024);
        assert!(solution.is_some());
        let solution = solution.unwrap();
        assert!(nov_6_2024.validate_solution(&solution).is_ok())
    }
}