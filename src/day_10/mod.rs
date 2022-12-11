use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn start() {
    let theday = "day_10";
    let file_name = "example";
    // let file_name = "puzzle_input";

    println!("{}", theday);
    let file_name = format!(
        "/workspaces/DEV/adventofcode2022/src/{}/{}",
        theday, file_name
    );

    let file_open = File::open(file_name).unwrap();
    let file = BufReader::new(file_open);
    let mut lines_iter = file.lines().map(|l| l.unwrap());

    // for line in lines_iter {
    //     let s = line.chars().next().unwrap();
    //     println!("{}", s);
    // }

    for tick in 1..11 {
        let line = lines_iter.next().unwrap();
        let s = line.chars().next().unwrap();
        let x = line.split_whitespace().nth(1).unwrap();
        println!("{} {} {}", tick, s, x);
    }

    println!();
    println!("answer part 1 = {}", "test");
    println!();

    // println!();
    // println!("answer part 2 = {}", sum);
    // println!();
}
