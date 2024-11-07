/// Solvers for the puzzle
/// 

/// just a list of the words used to solve, in order
type LBPuzzleSolution = Vec<String>;

mod dictionary {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::{PathBuf, Path};
    use std::collections::HashSet;
    
    fn get_dictionary_file_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/words_alpha.txt")
    }

    pub fn load_hashset_dictionary() -> HashSet<String> {
        // load the dictionary file
        // Open the file
        println!("Loading English dictionary from file...");
        let file = File::open(get_dictionary_file_path()).unwrap();

        // Create a buffered reader
        let reader = io::BufReader::new(file);
        
        let mut words = HashSet::<String>::new();
        // Iterate over the lines in the file
        for line in reader.lines() {
            // Add each word to the set (unwrap here for simplicity, but in practice handle errors)
            let word = line.unwrap();
            let word = word.trim();
            words.insert(word.to_string());
        }
        println!("Loaded {} words.", words.len());
        
        words
    }
    
    #[cfg(test)]
    mod tests {
        use crate::solver::dictionary::load_hashset_dictionary;

        #[test]
        fn test_load_hashset_dictionary() {
            // just make sure the load function actually runs and the hashset size is correct
            let words = load_hashset_dictionary();
            
            assert_eq!(words.len(), 370104);
        }
    }
}

pub mod brute_force {
    use crate::LBPuzzle;
    use crate::solver::dictionary::load_hashset_dictionary;

    /// idiotic solver that just goes through every combo
    /// 
    /// going about this the easy stupid way first to get a baseline.
    /// no tries, no dynamic programming, no nothing.
    /// it doesn't try to find the best solution; it just returns the first valid solution
    /// it can find.
    pub fn solve_brute_force<const L: usize,const T: usize>(puzzle: &LBPuzzle<L,T>) {
        // load a hashmap set of all words
        let words = load_hashset_dictionary();
        
        for side in puzzle.sides() {
            println!("{:?}", side);
        }
    }
    
}