/// This solver finds a good puzzle solution quickly by expressing the problem as A* search.
/// It uses pre_dict's precomputed dictionary to reduce search area.
/// 
/// Here's how we express this as A*:
/// define:
/// - "coverage(v)" is the set of puzzle letters covered so far at vertex "v"
/// - "letter" is a given letter present on the puzzle
/// - "coverage(e)" is the number of _previously uncovered_ letters covered by edge "e"
/// - (L*S) is the total number of letters on the puzzle
/// 
/// our graph:
/// - vertex: a tuple of (letter, coverage(v))
/// - edge: an individual word, connecting from its first letter to its last letter
/// - edge weight: (L*S) - coverage(e). We subtract from (L*S) so that better words cost less and w>0
/// 
/// our heuristic(s):
/// - (L*S) - coverage(v).  We could be smarter and prefer easier letters to hard ones (maybe use
///   scrabble letter values?), but this is a good option to start with.
/// 
/// our search will be constrained such that we will not traverse more than max_words edges.
/// 
/// TODO: assess how good the solutions are. how often does it find optimal? will need to implement
/// an exhaustive search, probably with dijkstra, to assess.

use crate::solvers::pre_dict::smart_dict;
use std::collections::{HashMap, HashSet};
use std::collections::BinaryHeap;
use std::io::{self, BufRead};
use crate::{LBPuzzle, LBPuzzleSolution};

pub fn solve_a_star<const L: usize,const S: usize>(puzzle: &LBPuzzle<L, S>) -> Option<LBPuzzleSolution> {
    let dict = smart_dict::SmartDictionary::new(&puzzle);
    
    None
}

