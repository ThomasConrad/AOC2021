extern crate ndarray;
use std::time::Instant;
use std::fs;
use std::path::Path;
use ndarray::prelude::*;

fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = fs::read_to_string(filename).unwrap();
    return file.split("\n\n").map(|s| s.to_string()).collect();
}
fn main() {
    let now = Instant::now();
    let lines = read_file("input.txt");
    let seq:Vec<i32> = lines[0].split(",")
                                .map(|s| s.to_string()).map(|s| s.parse::<i32>().unwrap())
                                .collect();

    //Construct all the boards
    let mut boards = vec![[[0 ; 5] ; 5]; lines.len()-1];
    for (i,line) in lines.iter().skip(1).enumerate(){
        let cols: Vec<String> = line.split("\n").map(|s| s.to_string()).collect();
        for (j,col) in cols.iter().enumerate(){
            let items: Vec<i32> = col.split_whitespace().map(|s| s.to_string())
                                .map(|s| s.trim().parse::<i32>().unwrap()).collect();
            for (k,item) in items.iter().enumerate(){
                boards[i][j][k] = *item;
            }
        }
    }
    //
    let rowsum = Array::<i32,_>::ones((5,1));
    let mut a = Array::zeros((5,5));
    let mut endnum = 0;
    'outer: for num in seq.iter(){

        for i in 0..boards.len(){
            //Set to -1
            for j in 0..5{
                for k in 0..5{
                    if boards[i][j][k] == *num{
                        boards[i][j][k] = -1;
                    }
                }
            }
            //Check if winning
            a = arr2(&boards[i]);
            //Rows
            for val in a.dot(&rowsum).iter(){
                if *val == -5{
                    //println!("NUMBER {} has won",i)
                    endnum = *num;
                    break 'outer;
                }
            }
            //Lines
            for val in rowsum.t().dot(&a).iter(){
                if *val == -5{
                    //println!("NUMBER {} has won",i);
                    endnum = *num;
                    break 'outer;
                }
            }

        }
    }
    let mut sum = 0;
    for n in a.iter(){
        if *n != -1{
            sum += n;
        }
    }
    println!("{}",sum*endnum);
    println!("it took {:.2?} seconds",now.elapsed());

}