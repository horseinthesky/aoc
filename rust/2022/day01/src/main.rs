fn main() {
    let mut elves = Vec::new();

    let mut sum = 0;
    for item in include_str!("input.txt").lines() {
        if item.is_empty() {
            elves.push(sum);
            sum = 0;
        } else {
            sum += item.parse::<u64>().ok().unwrap()
        }
    }

    elves.sort();
    elves.reverse();

    println!("{}", elves.iter().max().unwrap());
    println!("{}", elves[..3].iter().sum::<u64>());
}
