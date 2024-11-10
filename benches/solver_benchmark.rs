use criterion::{black_box, criterion_group, criterion_main, Criterion};
use letterboxed_solver::{NYTBoxPuzzle, LBPuzzle};
use letterboxed_solver::solvers::{brute_force, pre_dict, a_star};

fn get_nyt_example() -> NYTBoxPuzzle {
    // nov_6_2024    
    NYTBoxPuzzle::from_str(6, "erb uln imk jav").unwrap()
}

fn benchmark_brute_force(c: &mut Criterion) {
    let nov_6_2024 = get_nyt_example();
    println!("{:?}", nov_6_2024);

    c.bench_function("my_function", |b| b.iter(|| {
        let result = brute_force::solve_brute_force(black_box(&nov_6_2024));
        dbg!(&result);
    }));
}

fn benchmark_pre_dict(c: &mut Criterion) {
    let nov_6_2024 = get_nyt_example();
    println!("{:?}", nov_6_2024);

    c.bench_function("my_function", |b| b.iter(|| {
        let result = pre_dict::solve_pre_dict(black_box(&nov_6_2024));
        dbg!(&result);
    }));
}


fn benchmark_a_star(c: &mut Criterion) {
    let puzzle = get_nyt_example();
    println!("{}", puzzle);

    c.bench_function("a star", |b| b.iter(|| {
        a_star::solve_a_star(black_box(&puzzle));
    }));
}

// a star is the only one i care about
criterion_group!(benches, benchmark_a_star);
criterion_main!(benches);
