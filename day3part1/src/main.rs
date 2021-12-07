use std::fs;
use std::path::Path;

fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::read_to_string(filename).unwrap();
    return file.split("\n").map(|s| s.to_string()).collect();
}

fn to_decimal(arg: Vec<i32>) -> i32{
    let mut out = 0;
    for i in 0..arg.len(){
        out += arg[i]*(2 as i32).pow((arg.len()-i-1) as u32);
    }
    out
}

fn main() {
    let lines = read_file("input.txt");
    let n = lines.len();
    let mut v = vec![0.0;lines[0].len()];
    for line in lines{
        for (i,s) in line.chars().enumerate(){
            v[i] += ((s as i32 - 0x30) as f32)/(n as f32);
        }
    }    
    let end: Vec<i32> = v.iter().map(|x| x.round() as i32).collect();
    let endinv: Vec<i32> = end.iter().map(|x| (x-1).abs()).collect();
    println!("{}",to_decimal(end)*to_decimal(endinv));
}