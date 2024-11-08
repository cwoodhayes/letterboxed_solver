# Letterboxed Solver

This crate includes a solver for the [NYT Letter Boxed](https://www.nytimes.com/puzzles/letter-boxed) puzzle, along
with more general versions of that puzzle with larger & more sides.

I made it as a simple fun project to learn to write Rust, so
my priority has been more towards my own learning than towards
general cleanliness.  

Currently it doesn't generate the _best_ solution; just _a_ solution.
Also I don't have access to the same dictionary as NYT uses, so sometimes
the words here aren't included there :)

## Usage
```bash
# solve the puzzle from Nov 7, 2024, which allows 5 words
letterboxed_solver "vro wal eth bdi" 5
# output is something like:
# SOLUTION: ["iliahi", "iliad", "deliverable", "elbowboard", "deverbative"]
```

## Credits
My dictionary of english words was taken from 
[dwyl's english words repo](https://github.com/dwyl/english-words/). 

I put this together once I got through Ch. 14 of 
[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

And thanks to my partner berit for getting me into all these NYT puzzles :)