use log::debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use trie_rs::{Trie, TrieBuilder};

pub fn get_dictionary_from_file(path: &str) -> BufReader<File> {
    debug!("Loading English dictionary from file...");
    let p = Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("resources/dictionaries/{}", path));
    let file = File::open(p).unwrap();

    // Create a buffered reader
    BufReader::new(file)
}

pub fn get_dictionary_file_reader() -> BufReader<File> {
    get_dictionary_from_file("5000_common.txt")
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
        if word.len() > longest_word {
            longest_word = word.len();
        }
        words.push(word.to_string());
    }
    debug!(
        "Loaded {} words (longest {}). Building trie...",
        n_words, longest_word
    );
    let words = words.build();
    debug!("Trie built.");

    (words, n_words)
}

#[cfg(test)]
mod tests {
    use super::load_trie_dictionary;

    #[test]
    fn test_load_trie_dictionary() {
        // just make sure the load function actually runs and the hashset size is correct
        let (_, n_words) = load_trie_dictionary();

        assert_eq!(n_words, 370104);
    }
}

pub mod smart_dict {
    use crate::dictionary;
    use crate::LBPuzzle;
    use log::info;
    use std::collections::{HashMap, HashSet};
    use std::io::BufRead;
    use std::rc::Rc;

    pub(crate) struct _Builder(HashMap<char, Vec<Rc<String>>>);

    impl _Builder {
        /// Sorts all the letters in the dict by length. should be called once after everything's added.
        fn _sort(&mut self) {
            for words in self.0.values_mut() {
                words.sort_unstable_by(|w1, w2| w2.len().cmp(&w1.len()));
            }
        }

        fn _add_word(&mut self, word: String) {
            let first_letter = word
                .chars()
                .next()
                .expect("Shouldn't get an empty word here.");
            self.0
                .entry(first_letter)
                .or_insert(Vec::<Rc<String>>::new())
                .push(Rc::new(word));
        }

        /// get a flat version of all words in the dictionary, WITH each word given an index
        /// these indices are stable unless you call _add() or _sort() (which are only used by new())
        pub fn get_flat_indexed(&self) -> Vec<(usize, Rc<String>)> {
            // indexing scheme for all is just "whatever the index is in flat map
            // TODO change to btree so ordering is semantic and we have indices more naturally
            let noidx = self
                .0
                .iter()
                .flat_map(|(_, words)| words.iter().cloned())
                .collect::<Vec<Rc<String>>>();
            noidx.iter().cloned().enumerate().collect()
        }

        pub fn take_map(self) -> HashMap<char, Vec<Rc<String>>> {
            self.0
        }

        /// Load in the words in the dictionary, but filter them such that:
        ///     - only letters which are on the box can be included
        ///     - letters can only be followed by letters on the other sides
        ///     - words are >3 letters
        pub fn new<const S: usize, const L: usize>(puzzle: &LBPuzzle<S, L>) -> Self {
            let reader = dictionary::get_dictionary_file_reader();

            // precompute valid word hashes
            let mut side_to_valids: Vec<HashSet<char>> = Vec::new();
            for side_i in 0..S {
                side_to_valids.push(puzzle.valid_letters((side_i * L) as i32))
            }
            let all_valids = puzzle.valid_letters(-1);

            let idx_to_valids =
                |idx: i32| side_to_valids.get(idx as usize / L).unwrap_or(&all_valids);

            // bookkeeping vars
            let mut dictionary = Self(HashMap::new());
            let mut n_words: u32 = 0;

            let mut n_valid_words: u32 = 0;
            let mut longest_word = 0;

            // Iterate over the lines in the file
            'lines: for line in reader.lines() {
                // Add each word to the set (unwrap here for simplicity, but in practice handle errors)
                n_words += 1;
                let line = line.unwrap();
                let word = line.trim();
                if word.len() > longest_word {
                    longest_word = word.len();
                }

                // evaluate the conditions described above
                if word.len() < 3 {
                    continue 'lines;
                }
                let mut prev_letter_idx = -1;
                for letter in word.chars() {
                    if !idx_to_valids(prev_letter_idx).contains(&letter) {
                        continue 'lines;
                    }
                    // todo make valids a map to index so i don't have to do this
                    let new_idx = puzzle
                        .all_letters()
                        .chars()
                        .position(|c| c == letter)
                        .expect("letter must exist") as i32;
                    prev_letter_idx = new_idx;
                }
                // if we get here, the word is valid
                n_valid_words += 1;
                dictionary._add_word(word.to_string());
            }

            #[cfg(debug_assertions)]
            info!(
                "Loaded ({}/{}) words (longest {}). Sorting...",
                n_valid_words, n_words, longest_word
            );
            dictionary._sort();
            #[cfg(debug_assertions)]
            info!("Dictionary built.");

            dictionary
        }
    }

    /// A dictionary which only contains the words & information we actually need to
    /// evaluate a specific puzzle.
    pub struct SmartDictionary {
        _map: HashMap<char, Vec<Rc<String>>>,
        _flat: Vec<(usize, Rc<String>)>,
    }

    impl SmartDictionary {
        /// create the smart dictionary object
        pub fn new<const S: usize, const L: usize>(puzzle: &LBPuzzle<S, L>) -> Self {
            let builder = _Builder::new(puzzle);

            Self {
                _flat: builder.get_flat_indexed(),
                _map: builder.take_map(),
            }
        }

        /// get all entries under a given letter, or a flattened version with all words.
        pub fn get(&self, c: char) -> Option<&Vec<Rc<String>>> {
            self._map.get(&c)
        }

        /// get a flat version of all words in the dictionary.
        #[deprecated(since = "0.1.0", note = "use get_flat_indexed instead")]
        pub fn get_flat(&self) -> Vec<Rc<String>> {
            self._flat.iter().map(|(_, w)| w.clone()).collect()
        }

        /// get a flat version of all words in the dictionary alongside their indices
        pub fn get_flat_indexed(&self) -> &Vec<(usize, Rc<String>)> {
            // TODO change to btree so we don't need to actually call get_flat() here & so
            // ordering is semantic
            &self._flat
        }

        /// get all words under a letter, with each word given a globally unique index
        /// these indices are stable unless you call _add() or _sort() (which are only used by new())
        pub fn get_indexed(&self, c: char) -> Option<Vec<(usize, Rc<String>)>> {
            // find the index of the first word under this letter in flat_indexed
            let letter_words = self.get(c)?;
            let first_idx = self
                .get_flat_indexed()
                .iter()
                .position(|(_, w)| w == &letter_words[0])?;

            // slice the flat vec to get the words under this letter
            let out = self
                ._flat
                .iter()
                .skip(first_idx)
                .take(letter_words.len())
                .cloned()
                .collect();
            Some(out)
        }

        /// get the word at a given stable index, per get_flat_indexed
        pub fn get_word_by_idx(&self, idx: usize) -> Option<Rc<String>> {
            Some(self.get_flat_indexed()[idx].1.clone())
        }

        /// get the total number of words in the dictionary
        pub fn len(&self) -> usize {
            self._flat.len()
        }
    }
}

#[cfg(test)]
mod smart_dict_tests {
    use super::smart_dict::SmartDictionary;
    use crate::NYTBoxPuzzle;

    #[test]
    fn test_precompute_dictionary() {
        let nov_6_2024 = NYTBoxPuzzle::from_str(6, "erb uln imk jav").unwrap();
        // just make sure the load function actually runs and the hashset size is correct
        let dict = SmartDictionary::new(&nov_6_2024);

        assert!(dict.len() < 370104);
    }

    #[test]
    fn test_order_letters() {
        // todo
    }
}
