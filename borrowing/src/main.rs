


fn main() {

    /*********************************
     * Borrowing (pointers/references)
     * - For if you want to use a value without taking ownership of it
     * - References -> pointers to the reference holding the pointer to the values location in mem
     * - This prevents 'data races' see rust book, section 4.2
     * - References go in and out of scope independently of the data they point to
     *********************************/

    let s1 = String::from("hello");
    let borrow = &s1;

    println!("{borrow}");
    println!("{s1}");
    //both prints work because ownership is not passed, but instead 'borrow' borrows the value that
    //s1 points to and owns
    
     {
         //borrows follow the same rules as ownership, but independent of the actual owner
         let second_borrow = &s1;

         //the second borrow is valid here
         println!("{second_borrow}");
     }

     //the second borrow is out of scope here
     //println!("{second_borrow}"); //causes panic but rust gives a detailed error message
    
}
