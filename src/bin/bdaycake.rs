#[path = "../words.rs"]
mod words;
use std::io;

fn main() {
    let s = io::stdin();
    let mut input = words::Words::new(s.lock());
    let n = input.parse_next::<usize>().unwrap();
    let mut counts = [0; 720];
    for _ in 0..n {
        let dm = input.parse_next::<usize>().unwrap();
        let d = dm / 100 - 1;
        let m = dm % 100 - 1;
        let i = m * 30 + d;
        counts[i] += 1;
        counts[i+360] += 1;
    }
    let mut bestv = isize::max_value();
    let mut besti = 0;
    for i in 0..360 {
        // No sum() in Rust 1.0 stable
        let mut v = 0;
        for x in counts[i..i+30].iter() {
            v = v + x;
        }
        if v < bestv {
            besti = i;
            bestv = v;
        }
    }
    let m = besti / 30;
    let d = besti % 30;
    println!("{:02}{:02}", d+1, m+1);
}
