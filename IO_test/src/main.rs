use std::env;

/********************************************************************
 * Program arguments are passed as such:
 * cargo run -- <arg1> <arg2> ...
 *
 * the double hyphen (--) indicates the following args are for
 * the program, not for cargo itself.
 ********************************************************************/

//type defined for expected command line args
#[derive(Debug)]
struct ArgsFormat {
    args: String,
}

//instantiate a struct populated with the command line arguments
fn build_args(mut args: Vec<String>) -> ArgsFormat {
    ArgsFormat {
        args: args.remove(0),
    }
}

fn main() {

    /***************************************************************
     * Collect Args
     * -------------------------------------------
     * args() returns an iterator and .collect turns an interator into
     * a collection 
     *****************************************************************/
    //try passing ownership here
    let args: ArgsFormat = build_args(env::args().collect());
    dbg!(args);

}


