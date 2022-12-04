use std::collections::{hash_map::RandomState, HashMap, HashSet};

fn priority(b: u8) -> usize {
    (b'a'..=b'z')
        .chain(b'A'..=b'Z')
        .zip(1..=52)
        .collect::<HashMap<_, _>>()[&b]
}

fn part1(s: &str) -> usize {
    s.lines()
        .map(|line| {
            let (first_half, second_half) = line.split_at(line.len() / 2);
            let i = first_half.find(&second_half.chars().collect::<Vec<_>>()[..]).unwrap();
            priority(first_half.as_bytes()[i])
        })
        .sum()
}

fn part2(s: &str) -> usize {
    s.lines()
        .map(|line| HashSet::from_iter(line.bytes()))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|s| {
            let [a, b, c] = s else { unreachable!() };
            let ab: HashSet<u8, RandomState> = HashSet::from_iter(a.intersection(b).cloned());
            priority(*ab.intersection(c).next().unwrap())
        })
        .sum()
}

fn main() {
    let s = include_str!("input.txt");
    let p1 = part1(&s);
    assert_eq!(p1, 7908);
    let p2 = part2(&s);
    assert_eq!(p2, 2838);
    println!("part 1 = {p1}");
    println!("part 2 = {p2}");
}
