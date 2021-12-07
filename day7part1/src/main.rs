use std::fs;
use std::path::Path;

fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::read_to_string(filename).unwrap();
    return file.split("\n").map(|s| s.to_string()).collect();
}

fn main() {
    let mut seq:Vec<i32> = read_file("input.txt")[0].split(",")
    .map(|s| s.parse::<i32>().unwrap()).collect();

    seq.sort();
    let median = seq[seq.len()/2];
    let dist: i32 = seq.iter().map(|x| (x-median).abs()).sum();
    println!("{}",dist);
}

//p = as.numeric(read.delim("input.txt",header=FALSE,sep=","))
//sum(abs(p-median(p)))