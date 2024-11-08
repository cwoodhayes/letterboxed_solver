use std::collections::{HashMap, HashSet};
/// Solver where a more specific dictionary is precomputed
/// to help narrow the word list down & enable smarter ordering
///
/// So. Here's the plan.
/// - compute a Trie dictionary which contains _only_ possible words on the box, by considering
///   the following constraints:
///     - only letters which are on the box can be included
///     - letters can only be followed by letters on the other sides
/// - start exploring the solution tree, _starting with the longest words in the dictionary_.

use std::io::BufRead;
use crate::{LBPuzzle, LBPuzzleSolution};
use super::dictionary;

struct _DictionaryByLength(HashMap<char, Vec<String>>);

impl _DictionaryByLength {
    /// Sorts all the letters in the dict by length. should be called once after everything's added.
    fn sort(&mut self) {
        for words in self.0.values_mut() {
            words.sort_unstable_by(|w1, w2| w2.len().cmp(&w1.len()));
        }
    }

    fn add_word(&mut self, word: String) {
        let first_letter = word.chars().next().expect("Shouldn't get an empty word here.");
        self.0.entry(first_letter).or_insert(Vec::<String>::new()).push(word);
    }

    /// get all entries under a given letter, or a flattened version with all words.
    fn get(&self, c: &char) -> Option<&Vec<String>> {
        self.0.get(c)
    }

    /// get a not-necessarily-sorted flat version of all words in the dictionary.
    fn get_flat(&self) -> Vec<String> {
        let mut flattened: Vec<String> = Vec::new();
        for words in self.0.values() {
            flattened.extend(words.clone());
        }
        flattened
    }
}


/// just make the dictionary, quick & dirty, without caring too much about runtime.
/// possibly i should do some abstraction here & overload that dictionary:: method instead later
/// filter such that:
///     - only letters which are on the box can be included
///     - letters can only be followed by letters on the other sides
///     - words are >3 letters
fn precompute_dict_naive<const L: usize, const S: usize>(puzzle: &LBPuzzle<L, S>) -> (_DictionaryByLength, u32) {
    let reader = dictionary::get_dictionary_file_reader();

    // precompute valid word hashes
    let mut side_to_valids: Vec<HashSet<char>> = Vec::new();
    for side_i in 0..S {
        side_to_valids.push(puzzle.valid_letters((side_i * L) as i32))
    }
    let all_valids = puzzle.valid_letters(-1);

    let idx_to_valids = | idx: i32 | {
        side_to_valids.get(idx as usize / L).unwrap_or(&all_valids)
    };

    // bookkeeping vars
    let mut dictionary = _DictionaryByLength(HashMap::new());
    let mut n_words: u32 = 0;
    let mut n_valid_words: u32 = 0;
    let mut longest_word = 0;
    // Iterate over the lines in the file
    'lines: for line in reader.lines() {
        // Add each word to the set (unwrap here for simplicity, but in practice handle errors)
        n_words += 1;
        let word = line.unwrap();
        let word = word.trim();
        if word.len() > longest_word { longest_word = word.len(); }

        // evaluate the conditions described above
        if word.len() < 3 { continue 'lines; }
        let mut prev_letter_idx = -1;
        for letter in word.chars() {
            if !idx_to_valids(prev_letter_idx).contains(&letter) {
                continue 'lines;
            }
            // todo make valids a map to index so i don't have to do this
            prev_letter_idx = word.chars().position(|c| c == letter).expect("letter must exist") as i32;
        }
        // if we get here, the word is valid
        n_valid_words += 1;
        dictionary.add_word(word.to_string());
    }

    println!("Loaded ({}/{}) words (longest {}). Sorting...", n_valid_words, n_words, longest_word);
    dictionary.sort();
    println!("Dictionary built.");

    (dictionary, n_valid_words)
}


#[cfg(test)]
mod tests {
    use crate::NYTBoxPuzzle;
    use crate::solvers::pre_dict::precompute_dict_naive;

    #[test]
    fn test_precompute_dictionary() {
        let nov_6_2024 = NYTBoxPuzzle::from_str(6, "erb uln imk jav").unwrap();
        // just make sure the load function actually runs and the hashset size is correct
        let (_, n_words) = precompute_dict_naive(&nov_6_2024);

        assert!(n_words < 370104);
    }

    #[test]
    fn test_order_letters() {
        // todo
    }
}

pub fn solve_pre_dict<const L: usize,const S: usize>(puzzle: &LBPuzzle<L, S>) -> Option<LBPuzzleSolution> {
    let (dict, _) = precompute_dict_naive(&puzzle);
    _solve_helper(&dict, puzzle, LBPuzzleSolution::new())
}

fn _solve_helper<const L: usize, const S: usize>(dict: &_DictionaryByLength, puzzle: &LBPuzzle<L, S>, words: LBPuzzleSolution) -> Option<LBPuzzleSolution> {
    // base cases:
    // we've run out of words
    if words.len() > puzzle.max_words { return None; };

    println!("Evaluating {:?}", words);

    // we've got a solution!
    if puzzle.validate_coverage(&words) {
        return Some(words);
    }

    // collect all the words that start with the ending letter of the previous word.
    // if there's no last word (ie this is the first call), then just use all words
    let matching_words = match words.last() {
        None => &dict.get_flat(),
        Some(word) => {
            let last_char = word.chars().last().expect("Shouldn't get an empty word here.");
            let words = dict.get(&last_char);
            // if there's nothing under this letter, then this solution is a dead end--return none.
            if words.is_none() { return None }
            words.unwrap()
        }
    };

    // now go through all those words & see if they make a solution.
    for word in matching_words {
        if words.contains(&word) {
            // we don't want any repeat words, cuz they're useless
            continue;
        }
        let mut new_words = words.clone() as LBPuzzleSolution;
        new_words.push(word.clone());
        let soln = _solve_helper(dict, puzzle, new_words);
        // return if we've found something! we are greedy.
        if soln.is_some() { return soln;}
    };

    None
}