use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn start() {

    let theday = "day_2";
    // let file_name = "example";
    let file_name = "puzzle_input";



    println!("{}", theday);
    let file_name = format!("/workspaces/DEV/adventofcode2022/src/{}/{}", theday, file_name);
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }

    let mut vec = Vec::new();
    if let Ok(lines) = read_lines(file_name) {
        for line in lines {
            let li = line.expect("CANNOT read line");
            vec.push(li);
        }
    }

    fn get_score(fight: &str) -> i32 {
        match fight {
            "AX" => 4,
            "AY" => 8,
            "AZ" => 3,
            "BX" => 1,
            "BY" => 5,
            "BZ" => 9,
            "CX" => 7,
            "CY" => 2,
            "CZ" => 6,
            _ => 0,
        }
    }

    let mut sum = 0;
    for item in &vec {
        // let it: Vec<String> = item.split_whitespace().map(str::to_string).collect();
        let it = item.split_whitespace().collect::<Vec<&str>>();
        // println!("{}", it.concat());
        let g = get_score(it.concat().as_str());
        sum += g;
        // println!("score {}", g);
    }

    println!();
    println!("answer part 1 = {}", sum);
    println!();

    fn get_score_2(fight: &str) -> i32 {
        match fight {
            "AX" => 3,
            "AY" => 4,
            "AZ" => 8,
            "BX" => 1,
            "BY" => 5,
            "BZ" => 9,
            "CX" => 2,
            "CY" => 6,
            "CZ" => 7,
            _ => 0
        }
    }

    sum = 0;
    for item in &vec {
        let it = item.split_whitespace().collect::<Vec<&str>>();
        let g = get_score_2(it.concat().as_str());
        sum += g;
    }

    println!();
    println!("answer part 2 = {}", sum);
    println!();

}
