use std::env;

/********************************************************************
 * Program arguments are passed as such:
 * cargo run -- <arg1> <arg2> ...
 *
 * the double hyphen (--) indicates the following args are for
 * the program, not for cargo itself.
 ********************************************************************/

fn main() {


    //collect args
    //args() returns an iterator and .collect turns an interator into
    //a collection
    let args: Vec<String> = env::args().collect();
    dbg!(args);
}


