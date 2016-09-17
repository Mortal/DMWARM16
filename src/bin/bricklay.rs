#[path = "../words.rs"]
mod words;
use std::io;

fn main() {
    let s = io::stdin();
    let mut input = words::Words::new(s.lock());
    let w = input.parse_next::<usize>().unwrap();
    let h = input.parse_next::<usize>().unwrap();
    let even = h / 2;
    let odd = h - even;
    let full = odd * (w / 2) + even * ((w - 1) / 2);
    let half = odd * (w % 2) + even * (1 + (w + 1) % 2);
    println!("{} {}", full, half);
}
