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

        /* Was going to do it recursively, but it seems string are meant to be
         * iterated over. The reasoning is that if they contain unicode 
         * characters, you can't as easily determine which is the next char */
        for chr in line_str.as_str().chars() {
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

        println!("");
    }
}
