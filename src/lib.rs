
/// Top-level representation of a puzzle definition.
/// Does not contain the answer to the puzzle--merely its definition.
///
/// NSides - the number of sides on the puzzle
/// NLetters - the number of letters per side
pub struct LBPuzzle<const NSIDES: usize, const NLETTERS: usize> {
    // the max number of words allowed for a correct puzzle solution
    max_words: usize,
    // the letters in the puzzle, as a nested array of sides
    sides: [[char; NLETTERS]; NSIDES]
}

#[derive(Debug, Clone)]
pub enum LBPuzzleError<'a> {
    InputError(&'a str),
}

type Result<T> = std::result::Result<T, LBPuzzleError<'static>>;

impl <const S: usize, const L: usize> LBPuzzle<S, L> {
    pub fn new(max_words: usize, sides: [[char; L]; S]) -> Self {
        LBPuzzle { max_words, sides }
    }

    /// constructs the LBPuzzle from a space-separated string
    pub fn from_str(max_words: usize, sides_str: &str) -> Result<Self> {

        let mut sides = [[' '; L]; S];

        let _s_vec : Vec<&str> = sides_str.split_whitespace().collect();
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
}


/// The standard Letter Boxed puzzle from NYT.
/// just a square with 3 letters per side.
pub type NYTBoxPuzzle = LBPuzzle<4, 3>;


#[cfg(test)]
mod tests {
    use super::*;
    
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
    
}
