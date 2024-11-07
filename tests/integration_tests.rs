// basic integration tests for the module

// define some example input structs
mod nyt_examples {
    use letterboxed_solver::solver::brute_force;
    use letterboxed_solver::NYTBoxPuzzle;

    #[test]
    fn test_example() {
        let nov_6_2024 = NYTBoxPuzzle::from_str(6, "erb uln imk jav");
        let nov_6_2024 = nov_6_2024.unwrap();

        // todo make api and write big tests against it
        brute_force::solve_brute_force(&nov_6_2024);
    }
}