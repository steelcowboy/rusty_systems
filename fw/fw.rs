use std::env;

fn main() {
    let mut num_index = 0;

    // Make the command line arguments into a vector
    let args: Vec<_> = env::args().collect();
    
    /* Iterate over each argument, making tuples of (index, arg). Try casting the
     * result into something, and if it is something set the index as appropriate */
    /* NOTE: Using args.into_iter() would consume self (i.e. args) because into_iter
     * takes self as a parameter, whereas iter() takes &self as a parameter */
    if let Some(tup) = args.iter().enumerate().find(|ref arg| arg.1 == "-n") {
        num_index = tup.0;
    };

    if num_index != 0 {
    // TODO: Make sure next argument after flag is valid number
        println!("Found n flag at index {}. Next arg is {}", num_index, args[num_index + 1]);
    }
}
