use criterion::{black_box, criterion_group, criterion_main, Criterion};
use letterboxed_solver::{solvers::brute_force, NYTBoxPuzzle};
use letterboxed_solver::solvers::brute_force::solve_brute_force;
use letterboxed_solver::solvers::pre_dict::solve_pre_dict;

fn benchmark_brute_force(c: &mut Criterion) {
    let nov_6_2024 = NYTBoxPuzzle::from_str(6, "erb uln imk jav").unwrap();
    println!("{:?}", nov_6_2024);

    c.bench_function("my_function", |b| b.iter(|| {
        let result = solve_brute_force(black_box(&nov_6_2024));
        dbg!(&result);
    }));
}

fn benchmark_pre_dict(c: &mut Criterion) {
    let nov_6_2024 = NYTBoxPuzzle::from_str(6, "erb uln imk jav").unwrap();
    println!("{:?}", nov_6_2024);

    c.bench_function("my_function", |b| b.iter(|| {
        let result = solve_pre_dict(black_box(&nov_6_2024));
        dbg!(&result);
    }));
}

criterion_group!(benches, benchmark_pre_dict);
criterion_main!(benches);
