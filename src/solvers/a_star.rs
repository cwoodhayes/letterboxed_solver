//! This solver finds a good puzzle solution quickly by expressing the problem as A* search.
//! It uses pre_dict's precomputed dictionary to reduce search area.
//!
//! Here's how we express this as A*:
//! define:
//! - "coverage(v)" is the set of puzzle letters covered so far at vertex "v"
//! - "letter" is a given letter present on the puzzle
//! - "coverage(e)" is the set of _previously uncovered_ letters covered by edge "e"
//! - (L*S) is the total number of letters on the puzzle
//!
//! our graph:
//! - vertex: a tuple of (letter, coverage(v))
//! - edge: an individual word, connecting from its first letter to its last letter
//! - edge weight: (L*S). We want to minimize the number of words in our solution, so each word weighs the same
//!   See below for why this constant is L*S.
//!
//! our heuristic:
//! - h(v) = (L*S) - |coverage(v)|
//!
//! This heuristic+edge weight combo is chosen to ensure that the heuristic is "admissible",
//! meaning it never overestimates the actual minimal cost to reach the goal.
//! A* needs h(v) to have this property to guarantee finding an optimal solution.
//!
//! Intuitively, our h(v) makes sense because we probably get closer to the goal the more letters we've covered.
//!
//! However, there's a problem--we're always at risk of reaching the goal in a single word.
//! If we leave the edge cost at 1, h(v) will pretty much always exceed it the cost of that one edge.
//! Therefore we set the edge weight to be (L*S) to make sure h(v) is always lower.
//!
//! Note: search will be constrained such that we will not traverse more than max_words edges.
//!
//! Note 2: that at some point we could be smarter and prefer easier letters to hard ones (maybe use
//! scrabble letter values?), but this is a good option to start with.
//!

use log::{debug, info};
use pathfinding::prelude::astar;
use std::collections::BTreeSet;
use std::hash::Hash;

use crate::dictionary::smart_dict;
use crate::{LBPuzzle, LBPuzzleSolution};

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Vertex {
    letter: Option<char>, // start character is None, all else has Some
    coverage: BTreeSet<char>,

    _words_path: Option<Vec<usize>>, // list of dictionary indices representing the words used
}

impl Vertex {
    fn new(letter: Option<char>, coverage: BTreeSet<char>, words_path: Option<Vec<usize>>) -> Self {
        let new = Self {
            letter,
            coverage,
            _words_path: words_path,
        };
        // # [cfg(debug_assertions)]
        debug!("{:?}", new);
        new
    }

    /// gets a new start vertex
    fn new_start() -> Self {
        Vertex::new(None, BTreeSet::new(), None)
    }

    /// returns all successor nodes, i.e. ending letters & coverages for all words with this starting letter
    fn successors<const L: usize, const S: usize>(
        &self,
        dict: &smart_dict::SmartDictionary,
        puzzle: &LBPuzzle<L, S>,
    ) -> Option<Vec<(Self, u32)>> {
        // BASE CASE: we've visited the max number of words
        if self._words_path.clone().unwrap_or_default().len() == puzzle.max_words {
            return None;
        }
        // gather all dictionary words that start with this letter
        let next_words = match self.letter {
            Some(l_) => dict.get_indexed(l_).unwrap_or(Vec::new()),
            None => dict.get_flat_indexed().clone(),
        };

        // for each, construct the next vertex & assign an edge weight & return
        let successors = next_words
            .into_iter()
            .map(|(idx, w)| -> (Self, u32) {
                // coverage(v) = coverage(v') + coverage(e)
                // i could do something clever here to save memory by caching identical coverages.
                // we'll see if we need it.
                let coverage_e: BTreeSet<char> = w.chars().collect();
                let coverage = self.coverage.union(&coverage_e).cloned().collect();
                let mut words_path = match &self._words_path {
                    Some(p) => p.clone(),
                    None => Vec::new(),
                };
                words_path.push(idx);

                let v = Vertex::new(w.chars().last(), coverage, Some(words_path));
                (v, (L * S) as u32) // all edges are weight L*S
            })
            .collect();
        Some(successors)
    }

    /// h(v) = (L*S) - coverage(v)
    fn heuristic<const L: usize, const S: usize>(&self, _puzzle: &LBPuzzle<L, S>) -> u32 {
        ((L * S) - self.coverage.len()) as u32
    }
}

/// Solves the puzzle using A* search.
pub fn solve_a_star<const L: usize, const S: usize>(
    puzzle: &LBPuzzle<L, S>,
) -> Option<LBPuzzleSolution> {
    let dict = smart_dict::SmartDictionary::new(&puzzle);
    _helper(&puzzle, &dict)
}

/// Helper function for A* search.
/// broken out separately for benchmarking purposes.
pub fn _helper<const L: usize, const S: usize>(
    puzzle: &LBPuzzle<L, S>,
    dict: &smart_dict::SmartDictionary,
) -> Option<LBPuzzleSolution> {
    let start = Vertex::new_start();
    let mut n_nodes_visited: u64 = 0;
    let mut n_edges_traversed: u64 = 0;

    // run the search
    let result = astar(
        &start,
        |v| {
            n_nodes_visited += 1;
            #[cfg(debug_assertions)]
            if (n_nodes_visited % 1000) == 0 {
                let cost = v._words_path.clone().unwrap_or_default().len();
                debug!("Nodes visited: {}...g(v)={}", n_nodes_visited, cost);
            }
            v.successors(&dict, puzzle).unwrap_or(Vec::new())
        },
        |v| {
            let heur = v.heuristic(&puzzle);
            n_edges_traversed += 1;
            #[cfg(debug_assertions)]
            if (n_edges_traversed % 100000) == 0 {
                let cost = v._words_path.clone().unwrap_or_default().len();
                info!("Edges traversed: {}...g(v)={}", n_edges_traversed, cost);
            }
            heur
        },
        |v| v.heuristic(&puzzle) == 0,
    );

    // parse the solution
    let path = match result {
        Some((path, cost)) => {
            if (path.len() - 1) != (cost / ((L * S) as u32)) as usize {
                // path is 1 node longer than cost (aka n_words) because of the start node.
                panic!(
                    "word len ({cost}) != path len ({}) -- the algo isn't working right",
                    path.len()
                );
            }
            Some(path)
        }
        None => None,
    };
    info!("solution: {:?}", &path);
    info!(
        "Nodes visited: {} | Edges traversed: {}",
        n_nodes_visited, n_edges_traversed
    );

    // convert from index path to words
    let idx_path = path?
        .last()?
        ._words_path
        .clone()
        .expect("This should have length");
    let word_path: Vec<String> = idx_path
        .iter()
        .map(|idx| dict.get_word_by_idx(*idx).unwrap().as_ref().clone())
        .collect();
    info!("Word path: {:?}", word_path);

    Some(word_path)
}
