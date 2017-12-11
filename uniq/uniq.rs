use std::io;
/* Since the StdinLock returned by stdin.lock() implements the BufRead trait
 * we must import it in order to use its functions (e.g. .lines() ) */ 
use std::io::BufRead;

fn main() {
    let mut last_line = String::new();

    let stdin = io::stdin();
    /* Lock stdin to get a locked reference to stdin. I think this is a mutex,
     * and it prevents stdin from being read unexpectedly in other places */
    let handle = stdin.lock();

    /* Lines will read in a line one by line, and strip away the trailing newline.
     * It will return a Result, for which we can match to a successful result or
     * an error. */
    for line in handle.lines() {
        let line_str = match line {
            Ok(line) => line,
            Err(err) => panic!("failed to read line: {}", err)
        };

        // Strings can be directly compared
        if line_str != last_line {
            println!("{}", line_str);
        }

        // Strings can also be directly assigned
        last_line = line_str;
    }
}
