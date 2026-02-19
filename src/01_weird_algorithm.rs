use std::io::{self, BufRead};
// Optimized for speed: reads all input at once into a buffer
fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read a single line and parse into a specific type (e.g., i64)
    let mut n: i64 = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Invalid input"),
        _ => return,
    };

    //logic loop
    //print n at each step until it becomes 1
    while n != 1 {
        print!("{} ", n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }

    //final step
    println!("1");
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
