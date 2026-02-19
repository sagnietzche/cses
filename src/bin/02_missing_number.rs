use std::io::{self, BufRead};
// Optimized for speed: reads all input at once into a buffer
fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read a single line and parse into a specific type (e.g., i64)
    let n: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();

    //read the second line
    let nums: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let original_sum = n * (n + 1) / 2;
    let given_sum: i64 = nums.iter().sum();
    println!("{}", original_sum - given_sum);
}

fn main() {
    solve();
}

//if a problem asks for 3 integers,on one line, can use this destructing as
//let line = lines.next().unwrap().unwrap();
//let mut parts = line.split_whitespace();
//let n: i64 = parts.next().unwrap().parse().unwrap();
//let m: i64 = parts.next().unwrap().parse().unwrap();
//let k: i64 = parts.next().unwrap().parse().unwrap();
