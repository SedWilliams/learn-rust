
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

//calls this module whenever we run `cargo test`
#[cfg(test)]
mod tests { //module::tests
            
    //import everything from the parent module (add function)
    use super::*;

    //calls this function whenever we run `cargo test`
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    //above function is our unit-function test
}
