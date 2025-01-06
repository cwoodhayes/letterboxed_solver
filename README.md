# Letterboxed Solver

This crate includes a solver for the [NYT Letter Boxed](https://www.nytimes.com/puzzles/letter-boxed) puzzle, along
with more general versions of that puzzle with larger & more sides.

I made it as a simple fun project to learn to write Rust, so
my priority has been more towards my own learning than towards
general cleanliness.

I don't have access to the same dictionary as NYT uses, so sometimes
the words here aren't valid there (such as "driveboat" in the example below)

## Usage
```bash
conor@pc:~$ # solve the puzzle from Nov 7, 2024, which allows 5 words
conor@pc:~$ cargo run "vro wal eth bdi" 5

PUZZLE: "vro wal eth bdi" (turns: 5)
SOLUTION: ["whiled", "driveboat"]

```

## Algorithm
The solver finds an optimal (i.e. shortest number of words possible) solution to the puzzle
by translating the problem into a graph search task, then applying `A*` search.  
Here's how I did it:

### Graph + heuristic definition

 define:
 - `coverage(v)` is the set of puzzle letters covered so far at vertex $v$
 - `letter` is a given letter present on the puzzle
 - `coverage(e)` is the set of _previously uncovered_ letters covered by edge $e$
 - `(L*S)` is the total number of letters on the puzzle

 our graph:
 - vertex `v`: a tuple of `(letter, coverage(v))`
 - edge `e`: an individual word, connecting from its first letter to its last letter
 - edge weight `|e|`: `1 <= |e| <= (L*S)`. We want to minimize the number of words in our solution,
   so each word weighs the same. For a guaranteed-optimal solution, we use `|e| = (L*S)`, but for faster suboptimal
   solutions, we can use lower values of `|e|`. (See below for explanation on why)

 our heuristic:
 - `h(v) = (L*S) - |coverage(v)|`

 This heuristic+edge weight combo is chosen to ensure that the heuristic is "admissible",
 meaning it never overestimates the actual cost to reach the goal.
 A* needs `h(v)` to have this property to guarantee finding an optimal solution.

 #### why the weird edge weight?
 Intuitively, `h(v) = (L*S) - |coverage(v)|` makes sense because we probably get closer to the goal the more letters we've covered.
 A naive approach would then be to set the edge weight `|e| = 1`.  
 However, this weight makes `h(v)` inadmissible--consider the case in which we cover the whole puzzle in a single lucky word. In that case, `h(v_0) = (L*S)` for start vertex `v_0`,
 but the actual path cost is `C = 1`.  
 We resolve this issue by setting `|e| = (L*S)` so that `h(v) >= C` for all `v`, since `min_possible_path_cost = (L*S) >= (L*S) - |coverage(v)|`.

 That said--this optimal solution takes more time to find (though still really fast for an NYT puzzle, about 25ms on 
 my laptop). An empirically faster, possibly suboptimal solution may be found by setting edge weight to a lower value.

 **Note**: search will be constrained such that we will not traverse more than `max_words` edges.
 
 **Note 2**: Before running A*, we first reduce the graph size by eliminating all invalid words from the dictionary, i.e.
 words that cannot possibly be formed using the arrangement of letters on the particular puzzle we're solving.


## Credits
My dictionary of english words was taken from 
[dwyl's english words repo](https://github.com/dwyl/english-words/). 

I put this together once I got through Ch. 14 of 
[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

Also thanks to my partner berit for getting me into all these NYT puzzles :)
