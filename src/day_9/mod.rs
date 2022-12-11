use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn start() {
    let theday = "day_9";
    let file_name = "example";
    // let file_name = "puzzle_input";

    println!("{}", theday);
    let file_name = format!(
        "/workspaces/DEV/adventofcode2022/src/{}/{}",
        theday, file_name
    );
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

    // function follow(H, T) {
    //     const dx = H.x - T.x;
    //     const dy = H.y - T.y;
    //     if (Math.abs(dx) === 2 || Math.abs(dy) === 2) {
    //         T.x += Math.sign(dx);
    //         T.y += Math.sign(dy);
    //     }
    // }
    type Point<T> = (T, T);
    fn follow(head: Point<i32>, mut tail: Point<i32>) {
        let dx = head.0 - tail.0;
        let dy = head.1 - tail.1;
        if dx.abs() == 2 || dy.abs() == 2 {
            tail.0 += dx / dx.abs();
            tail.1 += dy / dy.abs();
        }
    }

    let h: Point<i32> = (1, 1);
    let mut t: Point<i32> = (3, 2);
    follow(h, t);
    println!("{} {}", t.0, t.1);

    let mut sum: i64 = 0;
    for item in &vec {
        let len = item.len();
        let first: &str = &item[..len / 2];
        let second: &str = &item[len / 2..];
        // println!("{} --- {}", first, second);
        let bytes = first.as_bytes();
        let sbytes = second.as_bytes();

        for (_i, &it) in bytes.iter().enumerate() {
            let mut hit = false;
            // println!("{} {}", i, it);

            for (_j, &sit) in sbytes.iter().enumerate() {
                // println!("{},{} => {} {}", i, j, it, sit);
                if it == sit {
                    // println!("{} {}", j, it);
                    if it < 91 {
                        sum += i64::from(it - 38);
                    } else {
                        sum += i64::from(it - 96);
                    }
                    hit = true;
                    break;
                }
            }
            if hit {
                break;
            }
        }
    }

    println!();
    println!("answer part 1 = {}", sum);
    println!();

    for item in &vec {
        let len = item.len();
        let first: &str = &item[..len / 2];
        let second: &str = &item[len / 2..];
        // println!("{} --- {}", first, second);
        let bytes = first.as_bytes();
        let sbytes = second.as_bytes();

        for (_i, &it) in bytes.iter().enumerate() {
            let mut hit = false;
            // println!("{} {}", i, it);

            for (_j, &sit) in sbytes.iter().enumerate() {
                // println!("{},{} => {} {}", i, j, it, sit);
                if it == sit {
                    // println!("{} {}", j, it);
                    if it < 91 {
                        sum += i64::from(it - 38);
                    } else {
                        sum += i64::from(it - 96);
                    }
                    hit = true;
                    break;
                }
            }
            if hit {
                break;
            }
        }
    }

    // println!();
    // println!("answer part 2 = {}", sum);
    // println!();
}
