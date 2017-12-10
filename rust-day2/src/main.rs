use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let filepath = env::args().nth(1)
        .expect("usage: rust-day2 <path to file>");

    let f = File::open(filepath)
        .expect("File not found");

    let mut sum = 0;
    let reader = BufReader::new(&f);

    for line in reader.lines() {
        sum += row_checksum(line.unwrap());
    }

    println!("{}", sum);
}

fn row_checksum(row: String) -> i32 {
    let numbers = 0;
    for number_string in row.split_whitespace() {
    }

    return max - min;
}