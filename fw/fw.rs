use std::env;

fn main() {
    let mut num_index = 0;

    // Make the command line arguments into a vector
    let args: Vec<_> = env::args().collect();
    
    /* Iterate over each argument, making tuples of (index, arg). Try casting the
     * result into something, and if it is something set the index as appropriate */
    // FIXME: This scope is taking ownership of args, so I can't use it later
    if let Some(tup) = args.into_iter().enumerate().find(|ref arg| arg.1 == "-n") {
        num_index = tup.0;
    };

    if num_index != 0 {
        println!("Found n flag at index {}. Next arg is {}", num_index, args[num_index + 1]);
    }
}
