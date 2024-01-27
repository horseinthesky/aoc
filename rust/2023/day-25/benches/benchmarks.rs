use day_25::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part1_biscardi() {
    part1_biscardi::process(divan::black_box(include_str!(
        "../input1.txt",
    )))
    .unwrap();
}
