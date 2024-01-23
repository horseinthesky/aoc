use day_18::*;

use divan::AllocProfiler;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

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

#[divan::bench]
fn part1_vertices() {
    part1_vertices::process(divan::black_box(
        include_str!("../input1.txt",),
    ))
    .unwrap();
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}

#[divan::bench]
fn part2_biscardi() {
    part2_biscardi::process(divan::black_box(include_str!(
        "../input2.txt",
    )))
    .unwrap();
}
