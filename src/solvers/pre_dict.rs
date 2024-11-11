//! Solver where a more specific dictionary is precomputed
//! to help narrow the word list down & enable smarter ordering
//!
//! So. Here's the plan.
//! - compute a dictionary which contains _only_ possible words on the box, by considering
//!   the following constraints:
//!     - only letters which are on the box can be included
//!     - letters can only be followed by letters on the other sides
//! - start exploring the solution tree, _starting with the longest words in the dictionary_.

use crate::dictionary::smart_dict;
use crate::{LBPuzzle, LBPuzzleSolution};
use log::debug;

use super::SolverStrategy;

pub struct PreDictSolver<const L: usize, const S: usize> {}

impl<const L: usize, const S: usize> SolverStrategy<L, S> for PreDictSolver<L, S> {
    fn solve(&self, puzzle: &LBPuzzle<L, S>) -> Option<LBPuzzleSolution> {
        let dict = smart_dict::SmartDictionary::new(&puzzle);
        _solve_helper(&dict, puzzle, LBPuzzleSolution::new())
    }
}

fn _solve_helper<const L: usize, const S: usize>(
    dict: &smart_dict::SmartDictionary,
    puzzle: &LBPuzzle<L, S>,
    words: LBPuzzleSolution,
) -> Option<LBPuzzleSolution> {
    // base cases:
    // we've run out of words
    if words.len() > puzzle.max_words {
        return None;
    };

    debug!("Evaluating {:?}", words);

    // we've got a solution!
    if puzzle.validate_coverage(&words) {
        return Some(words);
    }

    // collect all the words that start with the ending letter of the previous word.
    // if there's no last word (ie this is the first call), then just use all words
    let matching_words = match words.last() {
        #[allow(deprecated)]
        None => &dict.get_flat(),
        Some(word) => {
            let last_char = word
                .chars()
                .last()
                .expect("Shouldn't get an empty word here.");
            let words = dict.get(last_char);
            // if there's nothing under this letter, then this solution is a dead end--return none.
            if words.is_none() {
                return None;
            }
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
        new_words.push((*word).as_ref().clone());
        let soln = _solve_helper(dict, puzzle, new_words);
        // return if we've found something! we are greedy.
        if soln.is_some() {
            return soln;
        }
    }

    None
}
