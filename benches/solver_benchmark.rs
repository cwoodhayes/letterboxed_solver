use criterion::{black_box, criterion_group, criterion_main, Criterion};
use letterboxed_solver::dictionary::smart_dict::SmartDictionary;
use letterboxed_solver::solvers::a_star;
use letterboxed_solver::NYTBoxPuzzle; // Ensure this path is correct

fn get_nyt_example() -> NYTBoxPuzzle {
    // nov_6_2024
    NYTBoxPuzzle::from_str(6, "erb uln imk jav").unwrap()
}

fn benchmark_a_star(c: &mut Criterion) {
    let puzzle = get_nyt_example();
    println!("{}", puzzle);

    c.bench_function("A* solver", |b| {
        b.iter(|| {
            a_star::solve_a_star(black_box(&puzzle));
        })
    });
}

fn benchmark_a_star_helper(c: &mut Criterion) {
    let puzzle = get_nyt_example();
    println!("{}", puzzle);
    let dict = SmartDictionary::new(&puzzle);

    c.bench_function("A* helper", |b| {
        b.iter(|| a_star::_helper(black_box(&puzzle), black_box(&dict)));
    });
}

fn benchmark_pre_dict_smart_dict(c: &mut Criterion) {
    let puzzle = get_nyt_example();
    println!("{}", puzzle);

    c.bench_function("building smart dict", |b| {
        b.iter(|| SmartDictionary::new(black_box(&puzzle)));
    });
}

criterion_group!(
    benches,
    benchmark_a_star,
    benchmark_pre_dict_smart_dict,
    benchmark_a_star_helper
);
criterion_main!(benches);
