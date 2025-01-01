use log::{debug, info};
use pathfinding::prelude::astar;
use std::collections::BTreeSet;
use std::hash::Hash;

use super::SolverStrategy;
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
        // debug!("{:?}", new);
        new
    }

    /// gets a new start vertex
    fn new_start() -> Self {
        Vertex::new(None, BTreeSet::new(), None)
    }
}

/// This solver finds a good puzzle solution quickly by expressing the problem as A* search.
/// It uses pre_dict's precomputed dictionary to reduce search area.
///
/// Here's how we express this as A*:
/// define:
/// - "coverage(v)" is the set of puzzle letters covered so far at vertex "v"
/// - "letter" is a given letter present on the puzzle
/// - "coverage(e)" is the set of _previously uncovered_ letters covered by edge "e"
/// - (L*S) is the total number of letters on the puzzle
///
/// our graph:
/// - vertex: a tuple of (letter, coverage(v))
/// - edge: an individual word, connecting from its first letter to its last letter
/// - edge weight: 1 <= e <= (L*S). We want to minimize the number of words in our solution, so each word weighs the same
///   See below for explanation on the value of e.
///
/// our heuristic:
/// - h(v) = (L*S) - |coverage(v)|
///
/// This heuristic+edge weight combo is chosen to ensure that the heuristic is "admissible",
/// meaning it never overestimates the actual minimal cost to reach the goal.
/// A* needs h(v) to have this property to guarantee finding an optimal solution.
///
/// Intuitively, our h(v) makes sense because we probably get closer to the goal the more letters we've covered.
///
/// However, there's a problem--we're always at risk of reaching the goal in a single word.
/// If we leave the edge cost at 1, h(v) will pretty much always exceed it.
/// So in order to find an optimal solution, we can set the edge weight to be (L*S) to make sure h(v) is always lower.
///
/// That said--this optimal solution takes more time to find. A much faster, typically suboptimal solution may be found by setting edge weight
/// to a lower value, under 1.
///
/// Note: search will be constrained such that we will not traverse more than max_words edges.
///
/// Note 2: that at some point we could be smarter and prefer easier letters to hard ones (maybe use
/// scrabble letter values?), but this is a good option to start with.
pub struct AStarSolver<const L: usize, const S: usize> {
    /// value between 1 and (L*S)
    edge_weight: u32,
}

impl<const L: usize, const S: usize> SolverStrategy<L, S> for AStarSolver<L, S> {
    fn solve(&self, puzzle: &LBPuzzle<L, S>) -> Option<LBPuzzleSolution> {
        let dict = smart_dict::SmartDictionary::new(&puzzle);
        self._helper(&puzzle, &dict)
    }
}

impl<const L: usize, const S: usize> AStarSolver<L, S> {
    /// edge_weight_factor is a value between 0 and 1
    /// it will set edge weight to some integer value between 1 and (L*S)
    pub fn new(edge_weight_factor: f32) -> Self {
        Self {
            edge_weight: (edge_weight_factor * (L * S) as f32).round() as u32,
        }
    }

    /// returns all successor nodes, i.e. ending letters & coverages for all words with this starting letter
    fn successors(
        &self,
        v: &Vertex,
        dict: &smart_dict::SmartDictionary,
        puzzle: &LBPuzzle<L, S>,
    ) -> Option<Vec<(Vertex, u32)>> {
        // BASE CASE: we've visited the max number of words
        if v._words_path.clone().unwrap_or_default().len() == puzzle.max_words {
            return None;
        }
        // gather all dictionary words that start with this letter
        let next_words = match v.letter {
            Some(l_) => dict.get_indexed(l_).unwrap_or(Vec::new()),
            None => dict.get_flat_indexed().clone(),
        };

        // for each, construct the next vertex & assign an edge weight & return
        let successors = next_words
            .into_iter()
            .map(|(idx, w)| -> (Vertex, u32) {
                // coverage(v) = coverage(v') + coverage(e)
                // i could do something clever here to save memory by caching identical coverages.
                // we'll see if we need it.
                let coverage_e: BTreeSet<char> = w.chars().collect();
                let coverage = v.coverage.union(&coverage_e).cloned().collect();
                let mut words_path = match &v._words_path {
                    Some(p) => p.clone(),
                    None => Vec::new(),
                };
                words_path.push(idx);

                let new_v = Vertex::new(w.chars().last(), coverage, Some(words_path));
                (new_v, self.edge_weight)
            })
            .collect();
        Some(successors)
    }

    /// h(v) = (L*S) - coverage(v)
    fn heuristic(&self, v: &Vertex, _puzzle: &LBPuzzle<L, S>) -> u32 {
        ((L * S) - v.coverage.len()) as u32
    }

    /// Helper function for A* search.
    /// broken out separately for benchmarking purposes.
    pub fn _helper(
        &self,
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
                // #[cfg(debug_assertions)]
                // if (n_nodes_visited % 1000) == 0 {
                //     let cost = v._words_path.clone().unwrap_or_default().len();
                //     debug!("Nodes visited: {}...g(v)={}", n_nodes_visited, cost);
                // }
                self.successors(&v, &dict, puzzle).unwrap_or(Vec::new())
            },
            |v| {
                let heur = self.heuristic(&v, &puzzle);
                n_edges_traversed += 1;
                // #[cfg(debug_assertions)]
                // if (n_edges_traversed % 100000) == 0 {
                //     let cost = v._words_path.clone().unwrap_or_default().len();
                //     info!("Edges traversed: {}...g(v)={}", n_edges_traversed, cost);
                // }
                heur
            },
            |v| self.heuristic(&v, &puzzle) == 0,
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
}
