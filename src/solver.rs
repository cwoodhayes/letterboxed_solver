/// Solvers for the puzzle
///

use crate::LBPuzzle;

mod dictionary {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::{PathBuf, Path};
    use trie_rs::{TrieBuilder, Trie};

    fn get_dictionary_file_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/words_alpha.txt")
    }

    pub fn load_trie_dictionary() -> (Trie<u8>, u32) {
        // load the dictionary file
        // Open the file
        println!("Loading English dictionary from file into trie...");
        let file = File::open(get_dictionary_file_path()).unwrap();

        // Create a buffered reader
        let reader = io::BufReader::new(file);
        
        let mut words = TrieBuilder::<u8>::new();
        let mut n_words: u32 = 0;
        let mut longest_word = 0;
        // Iterate over the lines in the file
        for line in reader.lines() {
            // Add each word to the set (unwrap here for simplicity, but in practice handle errors)
            n_words += 1;
            let word = line.unwrap();
            let word = word.trim();
            if word.len() > longest_word { longest_word = word.len(); }
            words.push(word.to_string());
        }
        println!("Loaded {} words (longest {}). Building trie...", n_words, longest_word);
        let words = words.build();
        println!("Trie built.");

        (words, n_words)
    }
    
    #[cfg(test)]
    mod tests {
        use crate::solver::dictionary::load_trie_dictionary;

        #[test]
        fn test_load_trie_dictionary() {
            // just make sure the load function actually runs and the hashset size is correct
            let (words, n_words) = load_trie_dictionary();
            
            assert_eq!(n_words, 370104);
        }
    }
}

pub mod brute_force {
    use crate::{LBPuzzle, LBPuzzleSolution};
    use crate::solver::dictionary::load_trie_dictionary;

    /// idiotic solver that just goes through every combo
    /// 
    /// going about this the easy stupid way first to get a baseline.
    /// no tries, no dynamic programming, no nothing.
    /// it doesn't try to find the best solution; it just returns the first valid solution
    /// it can find.
    pub fn solve_brute_force<const L: usize,const T: usize>(puzzle: &LBPuzzle<L,T>) -> LBPuzzleSolution {
        // load a hashmap set of all words
        let (words, _) = load_trie_dictionary();
        let mut solution = LBPuzzleSolution::new();
        solution.push(String::new());
        
        for side in puzzle.sides() {
            println!("{:?}", side);
        }

        solution
    }
    
}