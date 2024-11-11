# Letterboxed Solver

This crate includes a solver for the [NYT Letter Boxed](https://www.nytimes.com/puzzles/letter-boxed) puzzle, along
with more general versions of that puzzle with larger & more sides.

I made it as a simple fun project to learn to write Rust, so
my priority has been more towards my own learning than towards
general cleanliness.  

By default, the CLI utility uses A* to find an optimal solution. However, my A* implementation is configurable to
find suboptimal solutions for quicker runs.
Also I don't have access to the same dictionary as NYT uses, so sometimes
the words here aren't included there :)

## Usage
```bash
conor@co:~$ # solve the puzzle from Nov 7, 2024, which allows 5 words
conor@pc:~$ cargo run "vro wal eth bdi" 5

PUZZLE: "vro wal eth bdi" (turns: 5)
SOLUTION: ["whiled", "driveboat"]

```

## Credits
My dictionary of english words was taken from 
[dwyl's english words repo](https://github.com/dwyl/english-words/). 

I put this together once I got through Ch. 14 of 
[The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

Also thanks to my partner berit for getting me into all these NYT puzzles :)
