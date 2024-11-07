/// Solvers for the puzzle
///

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
    use std::collections::{HashSet, VecDeque};
    use crate::{LBPuzzle, LBPuzzleSolution};
    use crate::solver::dictionary::load_trie_dictionary;
    use trie_rs::Trie;

    #[derive(Debug)]
    struct _Solution {
        pub words: LBPuzzleSolution,
        // the index of the last-visited letter
        pub last_idx: usize,
        // all the letters we've visited before. always length L*S
        pub visited_letters: Vec<bool>
    }
    
    impl _Solution {
        /// generate a copy of this solution where we've ended the current word and have
        /// started the next one (though it still needs a next letter)
        pub fn end_word(&self) -> Self {
            // end this word by starting another empty one
            let mut words = self.words.clone();
            words.push(String::new());
            
            _Solution {
                words,
                last_idx: self.last_idx,
                visited_letters: self.visited_letters.clone()
            }
        }
        
        pub fn clone(&self) -> Self {
            _Solution {
                words: self.words.clone(),
                last_idx: self.last_idx,
                visited_letters: self.visited_letters.clone()
            }
        }
    }

    /// idiotic solver that just goes through every combo
    /// 
    /// going about this the easy stupid way first to get a baseline.
    /// no dynamic programming, no clever optimizations, no nothing. Just a ton of wasted memory on string allocs.
    /// it doesn't try to find the best solution; it just returns the first valid solution
    /// it can find by doing recursive breadth-first search on the entire tree of possibilities.
    pub fn solve_brute_force<const L: usize,const S: usize>(puzzle: &LBPuzzle<L, S>) -> Option<LBPuzzleSolution> {
        let (dict, _) = load_trie_dictionary();

        // may need to use linked list here instead due to allocating a huge block of contiguous mem but we'll see
        let mut solution_queue: VecDeque<_Solution> = VecDeque::new();
        
        println!("Initializing solutions...");

        // initialize our solution queue with solutions starting with each letter
        for (i, letter) in puzzle.all_letters().chars().enumerate() {
            let mut words = LBPuzzleSolution::new();
            words.push(letter.to_string());
            let visited_letters = vec![false; L*S];

            let soln = _Solution {
                words, last_idx: i, visited_letters
            };
            solution_queue.push_back(soln);
        };

        // now DFS over all possible options
        while let Some(mut soln) = solution_queue.pop_front() {
            // indicate that we've now visited this letter
            // (doing it here so I don't have to write it every time I push to the queue)
            soln.visited_letters[soln.last_idx] = true;
            println!("Visiting solution: {:?}...", soln.words);

            // cases
            let curr_word = soln.words.last().expect("There should always be a word.");
            // if our current letters make a word
            if dict.exact_match(curr_word) {
                // if we have a working solution, return it!
                if soln.visited_letters.iter().all(| _l | *_l ) {
                    println!("Solution found! {soln:#?}");
                    return Some(soln.words);
                }

                // otherwise, add this situation to the queue: the word ends here, and we start a new one.
                // we need to do this for every valid letter
                _add_all_valid_letters(&mut solution_queue, &dict, &puzzle, &soln.end_word());
            }
            // either way, if we have the ability to continue this word, let's try that too.
            _add_all_valid_letters(&mut solution_queue, &dict, &puzzle, &soln);
        }

        None
    }
    
    /// adds all letters that have possible future solutions to the queue
    fn _add_all_valid_letters<const L: usize, const S: usize>(solution_queue: &mut VecDeque<_Solution>, dict: &Trie<u8>, puzzle: &LBPuzzle<L, S>, soln_stub: &_Solution) {
        // yes i know this is inefficient, i said i was doing this the quick & dumb way to benchmark
        let curr_word = soln_stub.words.last().expect("There should always be a last word.");
        let mut letters = HashSet::<char>::new();
        // todo how do i correctly type hint the iterator and use that directly rather than collecting?
        let results: Vec<String> = dict.postfix_search(curr_word).collect();
        for postfix in results {
            letters.insert(postfix.chars().next().expect("There should always be a letter."));
        }
        
        // intersect our valid word letters with our available puzzle letters
        let puzzle_valid_letters = &puzzle.valid_letters(soln_stub.last_idx as i32);
        let possible_letters = letters.intersection(puzzle_valid_letters);
        
        // now we have a list of next letters -- lets add these to the solution queue
        for letter in possible_letters {
            let mut new_soln = soln_stub.clone();
            let w = new_soln.words.last_mut().expect("There should always be a word.");
            w.push(*letter);
            solution_queue.push_back(new_soln);
        }
    }
    
}