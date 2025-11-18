fn main() {
    println!("Hello, world!");

    /* available types */

    /* scalar */

    //signed ints: i8, i16, i32, i64, i128, isize
    let int: i32 = 42;

    //unsigned (negative) ints: u8, u16, u32, u64, u128, usize
    let int1: u8 = 255;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'R';

    /* compound */
    
    //prints 500
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    println!("{}", tuple.0);
    assert_eq!(tuple.0, 500);
    
    //prints 1
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", array[0]);
    assert_eq!(array[0], 1);

}
