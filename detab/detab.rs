/* Detab: A program to read in input, converting tabs to the appropriate number
 * of spaces -- just like expand(1) does */

use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line_str = match line {
            Ok(line) => line,
            Err(err) => panic!("failed to read line: {}", err)
        };
        let mut num = 0;

        for chr in line_str.chars() {
            if chr == '\t' {
                let num_iters = 8 - (num % 8);
                for _ in 0..num_iters {
                    print!(" ");
                }
                num = 0;
            } 
            else {
                print!("{}", chr);
                num += 1;
            }
        }

        println!();
    }
}
