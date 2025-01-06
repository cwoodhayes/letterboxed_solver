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
Here's my design:

### Graph + heuristic definition

 define:
 - `coverage(v)` is the set of puzzle letters covered so far at vertex `v`
 - `letter` is a given letter present on the puzzle
 - `coverage(e)` is the set of _previously uncovered_ letters covered by edge `e`
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

 #### why is each vertex a tuple?
 For instance, why not have each vertex be a word, or a letter?

 Defining each vertex to be a word, connected to possible subsequent words, is another viable solution. However, this graph representation fails to encode a key aspect of the puzzle: that if 2 words cover the same uncovered letters, and start and end with the same letters, they are equivalent in terms of game state/proximity to our objective.  Encoding this idea into the graph enables a faster search, by shrinking the search area. Note that this issue could be addressed with a clever heuristic, making this solution similarly viable to my implementation.

 Defining each vertex to be a letter, connected to possible subsequent letters, doesn't work, because correct solutions on this graph will almost certainly contain cycles, which aren't discoverable by standard A*. 

 Making each vertex a tuple of `(letter, coverage(v))`, where each letter is a start/end letter of a word, ensures that there are no optimal cyclic solutions on the graph; this is because any path containing a cycle can be made shorter (and still valid) by omitting the cycle (i.e. `bib - bib - bark` can be shortened to `bib bark`, and `brane - earn - nab - brane - early` can be shortened to `brane - early`).

 #### why the weird edge weight?
 Intuitively, `h(v) = (L*S) - |coverage(v)|` makes sense because we probably get closer to the goal the more letters we've covered.
 A naive approach would then be to set the edge weight `|e| = 1`.  
 
 However, this weight makes `h(v)` inadmissible--consider the case in which we cover the whole puzzle in a single lucky word. In that case, `h(v_0) = (L*S)` for start vertex `v_0`,
 but the actual path cost is `C = 1`.  
 
 We resolve this issue by setting `|e| = (L*S)` so that `h(v) >= C` for all `v`, since `min_possible_path_cost = (L*S) >= (L*S) - |coverage(v)|`.

 That said--this optimal solution takes more time to find (though still really fast for an NYT puzzle, about 5ms on 
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
