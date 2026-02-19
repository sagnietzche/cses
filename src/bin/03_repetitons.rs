use std::cmp;
use std::io::{self, BufRead};
// Optimized for speed: reads all input at once into a buffer
fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read a single line and parse into a specific type (e.g., i64)
    let s = match lines.next() {
        Some(Ok(line)) => line.trim().to_string(),
        _ => return,
    };

    if s.is_empty() {
        return;
    }

    //track repetitions
    let mut max_rep = 1;
    let mut current_rep = 1;

    //convert string to bytes
    let bytes = s.as_bytes();

    for i in 1..bytes.len() {
        if bytes[i] == bytes[i - 1] {
            current_rep += 1;
        } else {
            current_rep = 1;
        }

        //update the global maximum
        max_rep = cmp::max(max_rep, current_rep);
    }
    println!("{}", max_rep);
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
