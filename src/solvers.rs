/// Solvers for the puzzle
/// & associated code
///

pub mod brute_force;
pub mod pre_dict;

mod dictionary {
    use log::debug;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::path::{Path};
    use trie_rs::{TrieBuilder, Trie};

    pub fn get_dictionary_file_reader() -> BufReader<File> {
        debug!("Loading English dictionary from file...");
        let p = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/words_alpha.txt");
        let file = File::open(p).unwrap();

        // Create a buffered reader
        BufReader::new(file)
    }

    pub fn load_trie_dictionary() -> (Trie<u8>, u32) {
        let reader = get_dictionary_file_reader();
        
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
        debug!("Loaded {} words (longest {}). Building trie...", n_words, longest_word);
        let words = words.build();
        debug!("Trie built.");

        (words, n_words)
    }
    
    #[cfg(test)]
    mod tests {
        use crate::solvers::dictionary::load_trie_dictionary;

        #[test]
        fn test_load_trie_dictionary() {
            // just make sure the load function actually runs and the hashset size is correct
            let (_, n_words) = load_trie_dictionary();
            
            assert_eq!(n_words, 370104);
        }
    }
}
