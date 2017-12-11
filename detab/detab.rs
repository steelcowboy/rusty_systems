/* Detab: A program to read in input, converting tabs to the appropriate number
 * of spaces -- just like expand(1) does */

use std::io;
use std::io::BufRead;
use std::io::Write;

// We know this is the max a tab will be, so let's slice it!
const TAB: [u8; 8] = *b"        ";

fn main() {
    let stdin = io::stdin();

    let stdout_raw = io::stdout();
    let mut stdout = stdout_raw.lock();

    for line in stdin.lock().lines() {
        let line_str = match line {
            Ok(line) => line,
            Err(err) => panic!("failed to read line: {}", err)
        };

        let mut num = 0;

        for chr in line_str.chars() {
            if chr == '\t' {
                // Borrow a slice of TAB to get the correct number of spaces
                let _ = stdout.write_all(&TAB[num % 8..]);
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
