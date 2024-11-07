// basic integration tests for the module
use letterboxed_solver::LBPuzzle;

// define some example input structs
mod nyt_examples {
    use letterboxed_solver::NYTBoxPuzzle;

    #[test]
    fn test_example() {
        let nov_6_2024 = NYTBoxPuzzle::from_str(6, "erb uln imk jav");
        
        // todo make api and write big tests against it
    }
}