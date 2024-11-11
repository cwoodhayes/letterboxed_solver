//! Module containing strategies for solving puzzles.
//!
//! Each strategy is a trait with a single method `solve`, which takes a puzzle and returns a solution.
//!
//! The strategies are:
//! - `AStar`: Uses A* search with a custom heuristic.
//! - `BruteForce`: Uses a really stupid brute force approach to check all possible words.
//! - `PreDict`: Uses a precomputed dictionary to speed up the search, but still isn't that smart about it.

pub mod a_star;
pub mod brute_force;
pub mod pre_dict;

use crate::{LBPuzzle, LBPuzzleSolution};

/// Strategy for solving a puzzle
pub trait SolverStrategy {
    fn solve<const L: usize, const S: usize>(puzzle: &LBPuzzle<L, S>) -> Option<LBPuzzleSolution>;
}
