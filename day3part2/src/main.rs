use std::fs;
use std::path::Path;

fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::read_to_string(filename).unwrap();
    return file.split("\n").map(|s| s.to_string()).collect();
}

fn to_decimal(arg: &Vec<i32>) -> i32{
    let mut out = 0;
    for i in 0..arg.len(){
        out += arg[i]*(2 as i32).pow((arg.len()-i-1) as u32);
    }
    out
}

fn main() {
    let lines = read_file("input.txt");
    let n = lines.len();
    let len = lines[0].len();
    let mut data = vec![vec![0;len];n];
    for (i,line) in lines.iter().enumerate(){
        for (j,s) in line.chars().enumerate(){
            data[i][j] = (s as i32 - 0x30) as i32;
        }
    }

    //oxygen
    let mut oxstencil = vec![1;n];
    let mut count1;
    let mut sum = n;
    for i in 0..len{
        count1 = 0;
        let mut keep = 1;
        for j in 0..n{
            if oxstencil[j] == 1{
                count1 += data[j][i];
            }
        }
        if (count1 as f32) < (sum as f32/2.0) {
            keep = 0;
        }
        for j in 0..n{
            if data[j][i] != keep{
                oxstencil[j] = 0;
            }
        }
        sum = oxstencil.iter().sum();
        if sum == 1{
            break;
        }
    }

    //co2
    let mut co2stencil = vec![1;n];
    sum = n;
    for i in 0..len{
        count1 = 0;
        let mut keep = 1;
        for j in 0..n{
            if co2stencil[j] == 1{
                count1 += data[j][i];
            }
        }
        if (count1 as f32) >= (sum as f32/2.0) {
            keep = 0;
        }
        for j in 0..n{
            if data[j][i] != keep{
                co2stencil[j] = 0;
            }
        }
        /*for p in co2stencil.iter(){
            print!{"{}",p};
        }
        println!("");*/
        sum = co2stencil.iter().sum();
        if sum == 1{
            break;
        }
    }
    let oxindex = oxstencil.iter().position(|&r| r == 1).unwrap();
    let co2index = co2stencil.iter().position(|&r| r == 1).unwrap();
    
    println!("{}",to_decimal(&data[oxindex])*to_decimal(&data[co2index]));
}