#[path = "../words.rs"]
mod words;
use std::io;
use std::collections::HashMap;

fn fill(x: Vec<Option<f64>>) -> Vec<f64> {
    let n = x.len();
    x.iter().enumerate().map(|(i, v)| {
        match *v {
            Some(v) => v,
            None => ((if i == 0 { x[i+1] } else { x[i-1] }).unwrap() +
                     (if i == n-1 { x[i-1] } else { x[i+1] }).unwrap()) / 2.0,
        }}).collect()
}

fn main() {
    let s = io::stdin();
    let mut input = words::Words::new(s.lock());
    let w = input.parse_next::<usize>().unwrap();
    let h = input.parse_next::<usize>().unwrap();
    let b = input.parse_next::<usize>().unwrap();
    let n = input.parse_next::<usize>().unwrap();
    let board_and_buckets = (0..h).map(|_| {
        input.read_line().unwrap()[1..w+1].to_string() })
        .collect::<Vec<_>>();
    // No split_last in Rust 1.0 stable
    // let (buckets, board) = board_and_buckets.split_last().unwrap();
    let (board, bucketss) = board_and_buckets.split_at(board_and_buckets.len()-1);
    let ref buckets = bucketss[0];
    input.read_line().unwrap(); // Frame
    let bucket_scores = (0..b).map(|_| {
        (input.next().unwrap().unwrap().chars().next().unwrap(),
         input.parse_next::<f64>().unwrap()) })
        .collect::<HashMap<_, _>>();
    let queries = (0..n).map(|_| input.parse_next::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut scores = Vec::new();
    // Last line
    scores.push(fill(
        buckets.chars().map(|c| {
            if c == '|' { None } else { Some(*bucket_scores.get(&c).unwrap()) } })
        .collect()));
    for row in board.iter().rev() {
        let row = {
            let prev = scores.last().unwrap();
            fill(row.chars().enumerate().map(|(i, c)| {
                if c == '.' { None } else { Some(prev[i]) }})
                .collect())
        };
        scores.push(row);
    }
    // No sum() in Rust 1.0 stable
    let mut res = 0.0f64;
    for x in queries.iter().map(|i| scores.last().unwrap()[i-1]) {
        res += x;
    }
    println!("{}", res);
}
