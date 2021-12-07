use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    return buf.lines()
              .map(|l| l.expect("Could not parse line")) //Handle error
              //.map(|x| x.parse().unwrap()) //convert to int32
              .collect() //Make into vector
}

fn main() {
    let lines = read_file("input.txt");
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;


    for line in lines{
        let res: Vec<String>  = line.split(" ").map(|s| s.to_string()).collect();
        if res[0] == "forward"{
            horizontal += res[1].parse::<i32>().unwrap();
            depth += aim * res[1].parse::<i32>().unwrap();
        }
        else if res[0] == "down"{
            aim += res[1].parse::<i32>().unwrap();
        }
        else if res[0] == "up"{
            aim -= res[1].parse::<i32>().unwrap();
        }
    }
    println!("{}",depth*horizontal)
}