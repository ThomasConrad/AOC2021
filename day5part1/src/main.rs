use std::fs;
use std::path::Path;
use itertools::Itertools; 
use std::cmp::min;
use std::cmp::max;


fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::read_to_string(filename).unwrap();
    return file.split("\n").map(|s| s.to_string()).collect();
}
fn main() {
    let lines = read_file("input.txt");
    let mut x = Vec::new();
    let mut y = Vec::new();

    for line in lines.iter(){
        let read:Vec<Vec<i32>> = line.split(" -> ").map(|s: &str| s.split(",").map(|s| s.parse::<i32>().unwrap()).collect()).collect();
        for (prev,next) in read.iter().tuples(){
            //horizontal
            if prev[0] == next[0]{
                for i in min(prev[1],next[1])..max(prev[1],next[1])+1{
                    x.push(prev[0]);
                    y.push(i);
                }
            }
            //vertical
            if prev[1] == next[1]{
                for i in min(prev[0],next[0])..max(prev[0],next[0])+1{
                    x.push(i);
                    y.push(prev[1]);
                }
            }
        }
    }
    assert_eq!(x.len(), y.len()); //check everything is ok
    let maxx:usize = *x.iter().max().unwrap() as usize;
    let maxy:usize = *y.iter().max().unwrap() as usize;

    let mut ground = vec![vec![0;maxx+2];maxy+2];
    let it = x.iter().zip(y.iter());

    for (x, y) in it {
        ground[*x as usize][*y as usize] += 1;
    }
    let mut n = 0;
    for num in ground.iter().flatten(){
        if *num > 1{
            n += 1;
        }
    }
    println!("Found {} dangerous spots",n);
}