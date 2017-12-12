use std::env;
use std::process;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::collections::HashMap;

fn main() {
    // Temporary mutable variables for parsing command line
    let mut _num_index = 0;
    let mut _num_words = 10;

    // Make the command line arguments into a vector
    let mut args: Vec<_> = env::args().collect();
    let mut prog_name = String::new();
    
    /* Iterate over each argument, making tuples of (index, arg). Try casting the
     * result into something, and if it is something set the index as appropriate */
    /* NOTE: Using args.into_iter() would consume self (i.e. args) because into_iter
     * takes self as a parameter, whereas iter() takes &self as a parameter */
    if let Some(tup) = args.iter().enumerate().find(|ref arg| arg.1 == "-n") {
        _num_index = tup.0;
    };

    // We've figured out where the "-n" flag is, it's a constant now
    let num_index = _num_index;
    if num_index != 0 {
        if num_index + 1 >= args.len() {
            print_usage(&args[0]);
        }

        // This pattern matching sees if the result is an Ok or Err, and binds
        // the inner element to the corresponding name (either n or e)
        match args[num_index+1].parse::<i32>() {
            Ok(n) => { 
                _num_words = n;

                // Done with that data, let's make args only contain the filenames
                // we need to read now
                // NOTE: Need to remove them in reverse, since indexing changes
                args.remove(num_index+1);
                args.remove(num_index);
                prog_name = args.remove(0); // Don't need the program name anymore
            },
            Err(e) => {
                eprintln!("{}", e);
                // Note that we can't give an element of an array to a function
                // (i.e. it can't take ownership of that element) so we must pass
                // it by reference
                print_usage(&args[0]);
            }
        }
    }
    else {
        prog_name = args.remove(0);
    }

    if args.len() < 1 {
        print_usage(&prog_name);
    }

    // Number of words finalized, no more modifying
    let num_words = _num_words;
    let mut wordcounts: HashMap<String, usize> = HashMap::new();
    let mut total_words = 0;

    // into_iter() will consume args, which is slightly unnecssary as it would
    // be freed at the end of the program anyways, but why not?
    for file in args.into_iter() {
        total_words += analyze_file(file, &mut wordcounts);
    }

    println!("Top {} words (out of {}) are:", num_words, total_words);
    let mut sorted_results = wordcounts.into_iter();

    for _ in 0..num_words {
        let (word, count) = match sorted_results.next() {
            Some(t) => t,
            None => {
                break;
            },
        };
        println!("{:6} {}", count, word);
    }
    
}

fn print_usage(name: &String) {
    eprintln!("USAGE: {} [-n] [FILE]...", name);
    process::exit(-1);
}

fn analyze_file(file_name: String, map: &mut HashMap<String, usize>) -> usize {
    let file = match File::open(file_name) {
        Err(e) => {
            eprintln!("{}", e);
            return 0;
        },
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    let mut total_words = 0;
    
    for line in reader.lines() {
        let l = line.unwrap();

        for word in l.split(|c: char| !c.is_alphanumeric()) {
            let mut wcount = 1;
            let word = word.to_string();

            if word == "" {
                continue;
            }

            if let Some(n) = map.get(&word) {
                wcount += n;
            }

            total_words += 1;
            map.insert(word, wcount); 
        }
    }

    return total_words;
}
