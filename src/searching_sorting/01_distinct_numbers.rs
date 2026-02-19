use std::io::Read;
use std::io::{self};

fn solve() {
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read");

    let mut tokens = input.split_whitespace();
    let n: usize = tokens.next().unwrap().parse().expect("Invalid N");

    let mut values: Vec<i32> = tokens
        .take(n)
        .map(|s| s.parse().expect("Invalid Integer"))
        .collect();

    values.sort_unstable();

    values.dedup();

    println!("{}", values.len());
}

fn main() {
    solve();
}
