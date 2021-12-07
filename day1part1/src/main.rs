use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line")) //Handle error
        .map(|x| x.parse().unwrap()) //convert to int32
        .collect() //Make into vector
}

fn main() {
    let lines = read_file("input.txt");
    let mut amount = 0;
    for i in 1..lines.len(){
        if lines[i]-lines[i-1] > 0 {
            amount += 1;
        }
    }
    println!("{}",amount);
}