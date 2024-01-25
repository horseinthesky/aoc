use day_21::*;

use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(consts = [
        6,
        64
    ])]
fn part1<const N: usize>() {
    part1::process(
        divan::black_box(include_str!("../input1.txt")),
        divan::black_box(N),
    )
    .unwrap();
}

#[divan::bench(consts = [
        6,
        64
    ])]
fn part1_biscardi<const N: usize>() {
    part1_biscardi::process(
        divan::black_box(include_str!("../input1.txt")),
        divan::black_box(N),
    )
    .unwrap();
}

#[divan::bench(consts = [
        6,
        64
    ])]
fn part2<const N: usize>() {
    part2::process(
        divan::black_box(include_str!("../input2.txt",)),
        divan::black_box(N),
    )
    .unwrap();
}

#[divan::bench(consts = [
    6,
    64
])]
fn part2_biscardi<const N: usize>() {
    part2_biscardi::process(
        divan::black_box(include_str!("../input2.txt",)),
        divan::black_box(N),
    )
    .unwrap();
}

