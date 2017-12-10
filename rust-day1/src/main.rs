use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filepath = env::args().nth(1)
        .expect("usage: rust-day1 <path to file>");

    let mut f = File::open(filepath)
        .expect("File not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Could not read file");

    let mut sum = 0;
    for (index, number_char) in contents.chars().enumerate() {
        if number_char == next_char(index, &contents) {
           sum += number_char.to_digit(10).unwrap();
        }
    }

    println!("{}", sum);
}

fn next_char(index: usize, contents: &String) -> char {
    let content_length = contents.len();
    let next_index = (index + (content_length / 2)) % content_length;
    return contents.chars().nth(next_index).unwrap();
}