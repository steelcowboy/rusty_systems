use std::io;
use std::io::BufRead;

fn main() {
    let mut last_line = String::new();

    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => panic!("failed to read line: {}", err)
        };

        if line != last_line {
            println!("{}", line);
        }

        last_line = line;
    }
}
