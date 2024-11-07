use std::collections::HashSet;
use crate::LBPuzzleError::BadSolutionError;

pub mod solver;

/// Top-level representation of a puzzle definition.
/// Does not contain the answer to the puzzle--merely its definition.
///
/// NSides - the number of sides on the puzzle
/// NLetters - the number of letters per side
#[derive(Debug)]
pub struct LBPuzzle<const NSIDES: usize, const NLETTERS: usize> {
    // the max number of words allowed for a correct puzzle solution
    max_words: usize,
    // the letters in the puzzle, as a nested array of sides
    sides: [[char; NLETTERS]; NSIDES]
}

#[derive(Debug, Clone)]
pub enum LBPuzzleError<'a> {
    InputError(&'a str),
    BadSolutionError(String),
}

/// just a list of the words used to solve, in order
type LBPuzzleSolution = Vec<String>;

type Result<T> = std::result::Result<T, LBPuzzleError<'static>>;

/// The standard Letter Boxed puzzle from NYT.
/// just a square with 3 letters per side.
pub type NYTBoxPuzzle = LBPuzzle<4, 3>;


impl <const S: usize, const L: usize> LBPuzzle<S, L> {
    pub fn new(max_words: usize, sides: [[char; L]; S]) -> Self {
        LBPuzzle { max_words, sides }
    }

    /// constructs the LBPuzzle from a space-separated string
    pub fn from_str(max_words: usize, sides_str: &str) -> Result<Self> {
        let mut sides = [[' '; L]; S];

        let _s_vec: Vec<&str> = sides_str.split_whitespace().collect();
        // sanity check the number of sides
        if _s_vec.len() != S {
            return Err(LBPuzzleError::InputError("Wrong number of sides."));
        }

        // loop through and create the char array
        for (i, _l_vec) in _s_vec.iter().enumerate() {
            sides[i] = match _l_vec.chars().collect::<Vec<char>>().try_into() {
                Ok(s) => s,
                Err(_) => return Err(LBPuzzleError::InputError("wrong letters")),
            }
        }
        let puzzle = LBPuzzle::new(max_words, sides);
        Ok(puzzle)
    }

    /// get a copy of sides
    pub fn sides(&self) -> [[char; L]; S] {
        self.sides
    }

    /// get max words
    pub fn max_words(&self) -> usize {
        self.max_words
    }

    /// get a neat vector of all letters in the puzzle
    pub fn all_letters(&self) -> String {
        let mut all = String::new();
        for side in self.sides {
            all.extend(side);
        }
        all
    }

    /// the number of total letters in the puzzle (counting repeats, which I don't think usually
    /// exist anyhow)
    pub fn n_letters() -> usize {
        S * L
    }

    /// Return None if out of range.
    pub fn idx_to_side(&self, idx: i32) -> Option<i32> {
        if 0 <= idx && idx < Self::n_letters() as i32 {
            return Some(idx / L as i32)
        }
        None
    }

    /// returns true if the letter at index "idx" is on the side with index "side",
    pub fn is_idx_on_side(&self, idx: i32, side: i32) -> bool {
        self.idx_to_side(idx).unwrap_or(-1) == side
    }

    /// returns a HashSet of possible next letters
    pub fn valid_letters(&self, prev_idx: i32) -> HashSet<char> {
        let mut letters = HashSet::new();

        for (i, side) in self.sides().iter().enumerate() {
            if !self.is_idx_on_side(prev_idx, i as i32) {
                letters.extend(side.iter());
            }
        }

        letters
    }

    /// See if we can solve the puzzle given a solution
    pub fn validate_solution(&self, solution: &LBPuzzleSolution) -> Result<()> {
        // for NYT, all words must be 3 letters or more, so check that
        for word in solution {
            if word.len() < 3 {
                return Err(BadSolutionError(format!("{} is <3 letters long", word)));
            }
        }
        // merge the words into a simple sequence of letters
        let mut seq = solution.get(0).unwrap().clone();
        for word in &solution[1..] {
            if word.chars().next() != seq.chars().last() {
                return Err(BadSolutionError("Start & end letters don't match".to_string()));
            }
            seq.push_str(&word[1..]);
        }

        // validate that we can travel around the board with these letters,
        // AND that we touch all of them when we do.
        let mut visited_letters = [[false; L]; S];

        print!("Validated: ");
        let mut prev_side = -1;
        'letters: for letter in seq.chars() {
            'sides: for (i, side) in self.sides().iter().enumerate() {
                if i as i32 == prev_side {
                    continue 'sides;
                }
                let idx = side.iter().position(|_l| letter.eq(_l));
                if let Some(idx) = idx {
                    print!("{}", letter);
                    prev_side = i as i32;
                    visited_letters[i][idx] = true;
                    continue 'letters;
                }
            }
            return Err(BadSolutionError(format!("Failed to find letter {}", letter)));
        }

        // make sure we visited all the letters
        for side in visited_letters {
            if !side.iter().all(|&x| x) {
                return Err(BadSolutionError("Not all letters were used.".to_string()));
            }
        }

        println!("✅");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::LBPuzzle;
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
            assert!(nov_6_2024.validate_solution(&example).is_ok());
        }
        for example in invalids {
            println!("TEST: {:?}", example);
            let result = nov_6_2024.validate_solution(&example);
            println!("\n{:?}", result);
            assert!(result.is_err());
        }

    }

    #[test]
    fn test_from_str() {
        let sides_a = [
            ['e', 'r', 'b'],
            ['u', 'l', 'n'],
            ['i', 'm', 'k'],
            ['j', 'a', 'v']
        ];
        let string_a = String::from("erb uln imk jav");

        let puzzle = LBPuzzle::<4,3>::from_str(5, &string_a);

        assert!(puzzle.is_ok());
        let puzzle = puzzle.unwrap();
        assert_eq!(puzzle.max_words(), 5);
        assert_eq!(puzzle.sides(), sides_a);
    }

    #[test]
    fn test_index_side() {
        let puzzle = LBPuzzle::<4,3>::from_str(4, &"erb uln imk jav".to_string()).unwrap();
        assert!(puzzle.is_idx_on_side(0, 0));
        assert!(puzzle.is_idx_on_side(3, 1));
        assert!(puzzle.is_idx_on_side(2, 0));
        assert!(puzzle.is_idx_on_side(11, 3));

        assert_eq!(puzzle.idx_to_side(0).unwrap(), 0);
    }

}
