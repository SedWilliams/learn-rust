use std::env;
use std::fs;

//TODO update readme about what is held at the 0th index of args vector (its debug info)
//TODO implement tuple struct instead

/********************************************************************
 * Program arguments are passed as such:
 * cargo run -- <arg1> <arg2> ...
 *
 * the double hyphen (--) indicates the following args are for
 * the program, not for cargo itself.
 ********************************************************************/

//type defined for expected command line args
#[derive(Debug)]
struct ArgsFormatter {
    filename: String,
    expression: String,
}

//instantiate a struct populated with the command line arguments
fn build_args(mut args: Vec<String>) -> ArgsFormatter {
    ArgsFormatter {
        //these are both one because the 0th index is the program name
        //and when the previous arg is removed the next arg shifts to the previous index
        filename: args.remove(1),
        expression: args.remove(1),
    }
}

fn main() {

    /***************************************************************
     * Collect Args
     * -------------------------------------------
     * args() returns an iterator and .collect turns an interator into
     * a collection 
     *****************************************************************/
    //returns a collection that will be mutated later which is why the function above accepts it as
    //mut
    let args: ArgsFormatter = build_args(env::args().collect());
    //dbg!(args.filename);
    //dbg!(args.expression);
    
    let file_name = &args.filename;
    let expression = &args.expression;

    println!("Reading from file: {}", &file_name);
    println!("Searching for expression: {}", &expression);

    /***************************************************************
     * Read File
     * -------------------------------------------
     * read_to_string returns a Result enum that needs to be handled
     * with match or unwrap
     *****************************************************************/
    let file_content = fs::read_to_string(&file_name).expect("Could not read file to string");
    println!("File Content:\n{}", &file_content);
}


