# Advent of Code 2023

This year I've pre-set up a series of functionality for testing, benchmarking, and otherwise evaluating the performance of our Rust programs.

This includes the command `just work` which is passed a particular day and part and is the equivalent workflow of running all of these in a row and stopping if one fails, then re-starting the flow after changes.

```
cargo check
cargo nextest run
clippy-tracing --action check
cargo clippy
cargo bench
cargo flamegraph
```

## Quick setup

```shell
rustup install nightly
cargo install cargo-watch cargo-nextest cargo-generate flamegraph just
```

## Prepare for a new day

```shell
just create day-<n>
```

## Just

Just is used to partially document all tasks, so you (the person reading this) can see what commands we were running and perhaps run them yourself on your own codebase.

I also thought it would be neat to maybe have scripts that run flamegraphs for all of the days and parts easily, so that they could be checked in and viewable on github and generally make it easier to run and document the running of various tools.

```shell
cargo install just
```

## cargo-flamegraph

Flamegraphs are great for identifying what's taking up time in a program!

[cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph) will output an SVG that contains a flamegraph of the given program.

```shell
cargo install flamegraph
```

## Divan

Usually I use criterion for benchmarking but this year I wanted to try out a new benchmarking tool called [Divan](https://github.com/nvzqz/divan).

I don't have a strong reason for using [Divan over criterion](https://nikolaivazquez.com/blog/divan/#compared-to-criterion) other than to get to know the crate a bit better.

You can learn more about Divan in the [announcement post](https://nikolaivazquez.com/blog/divan/).

## cargo-nextest

[cargo-nextest](https://nexte.st/) is "a next-generation test runner for Rust projects". Basically that means it includes [an interesting execution model](https://nexte.st/book/how-it-works.html) than can be great for projects with a _lot_ of tests.

As of this year's AoC, **cargo-nextest** doesn't run doctests yet, so while that won't be an issue for us it is something to be aware of if you're using nextest in a "real project". (Basically that means you also run `cargo test --doc`).

cargo-nextest has what I consider [a positive relationship with the regular `cargo test`](https://nexte.st/book/how-it-works.html#contributing-features-back-to-cargo) and is rightfully a nice place to be experimenting with new testing UX. `cargo test` works well and `cargo nextest` is a forward-looking place for experimentation.

```shell
cargo install cargo-nextest
```

## tracing

The [tracing](https://docs.rs/tracing/0.1.40/tracing/index.html) crate is (in my opinion) the modern standard tracing and logging infrastructure tool in the Rust ecosystem. It is maintained by the same group of people working on the tokio async runtime and is extremely widely used.

For advent of code we could get away with using `println!` and `dbg!` but [tracing](https://docs.rs/tracing/0.1.40/tracing/index.html) offers not only rich span and event tracking, but also integration with other tools like [tracy](https://github.com/wolfpld/tracy).

## tracy

[tracy](https://github.com/wolfpld/tracy) is a profiling tool I'd like to get better at using. It is the least-known tool on this list for me.
- [tracing_tracy](https://docs.rs/tracing-tracy/0.10.4/tracing_tracy/index.html)
