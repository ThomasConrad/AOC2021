use std::fs;
use std::path::Path;

fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::read_to_string(filename).unwrap();
    return file.split("\n").map(|s| s.to_string()).collect();
}

fn main() {
    let seq:Vec<i32> = read_file("input.txt")[0].split(",")
    .map(|s| s.parse::<i32>().unwrap()).collect();
    let mut fish = vec![0;9];
    //fill fish into thingy
    for num in seq{
        fish[num as usize] += 1;
    }
    //Begin breeding
    for _ in 0..80{
        fish.rotate_left(1);
        fish[6] += fish[8]
    }
    let sum : i32= fish.iter().sum();
    println!("{}",sum);
}