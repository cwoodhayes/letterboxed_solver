/// Solvers for the puzzle
///

use crate::LBPuzzle;

/// just a list of the words used to solve, in order
type LBPuzzleSolution = Vec<String>;

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

/// See if we can solve the puzzle given a solution
pub fn validate_solution<const S: usize, const L: usize>(solution: &LBPuzzleSolution, puzzle: &LBPuzzle<S, L>) -> Result<(), String> {
    // for NYT, all words must be 3 letters or more, so check that
    for word in solution {
        if word.len() < 3 {
            return Err(String::from(format!("{} is <3 letters long", word)));
        }
    }
    // merge the words into a simple sequence of letters
    let mut seq= solution.get(0).unwrap().clone();
    for word in &solution[1..] {
        if word.chars().next() != seq.chars().last() {
            return Err(String::from("Start & end letters don't match"));
        }
        seq.push_str(&word[1..]);
    }

    // validate that we can travel around the board with these letters,
    // AND that we touch all of them when we do.
    let mut visited_letters = [[false; L]; S];

    print!("Validated: ");
    let mut prev_side = -1;
    'letters: for letter in seq.chars() {
        'sides: for (i, side) in puzzle.sides().iter().enumerate() {
            if i as i32 == prev_side {
                continue 'sides;
            }
            let idx = side.iter().position( |_l| letter.eq(_l) ); 
            if let Some(idx) = idx {
                print!("{}", letter);
                prev_side = i as i32;
                visited_letters[i][idx] = true;
                continue 'letters;
            }
        }
        return Err(String::from(format!("Failed to find letter {}", letter)));
    }
    
    // make sure we visited all the letters
    for side in visited_letters {
        if !side.iter().all(|&x| x ) {
            return Err(String::from("Not all letters were used."));
        }
    }
    
    
    println!("✅");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NYTBoxPuzzle;

    #[test]
    fn test_validate_solution() {
        // known solution & puzzle from https://nytletterboxed.com/letter-boxed-november-06-2024-answers/
        let nov_6_2024 = NYTBoxPuzzle::from_str(6, "erb uln imk jav");
        let nov_6_2024 = nov_6_2024.unwrap();
        let valids = [
            vec!(
                "juvenile".to_string(),
                "embark".to_string()
            ),
            vec!(
                "murk".to_string(),
                "kanji".to_string(),
                "inviable".to_string()
            )
        ];
        let invalids = [
            vec!(
                "poop".to_string()
            ),
            vec!(
                "ju".to_string(),
                "uv".to_string(),
            ),
            vec!(
                "juvenile".to_string(),
            ),
        ];

        for example in valids {
            println!("TEST: {:?}", example);
            assert_eq!(validate_solution(&example, &nov_6_2024), Ok(()));
        }
        for example in invalids {
            println!("TEST: {:?}", example);
            let result = validate_solution(&example, &nov_6_2024);
            println!("\n{:?}", result);
            assert!(result.is_err());
        }

    }
}

pub mod brute_force {
    use crate::LBPuzzle;
    use crate::solver::dictionary::load_trie_dictionary;
    use crate::solver::LBPuzzleSolution;

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