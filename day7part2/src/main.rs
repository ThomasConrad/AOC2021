use std::fs;
use std::path::Path;

fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::read_to_string(filename).unwrap();
    return file.split("\n").map(|s| s.to_string()).collect();
}

fn main() {
    let mut seq:Vec<i32> = read_file("input.txt")[0].split(",")
    .map(|s| s.parse::<i32>().unwrap()).collect();

    let mean: i32 = ((seq.iter().sum::<i32>() as f32 / seq.len() as f32) as f32).floor() as i32;

    let mut t: Vec<i32> = Vec::new();
    for (index,i) in (0..1).enumerate(){
        t.push(seq.iter().map(|x| (x-mean-i).abs()).map(|x| x*(x+1)/2).sum());
    }
    t.sort();
    println!("{}",t[0]);
}