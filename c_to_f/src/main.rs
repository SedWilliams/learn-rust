use std::io;
use c_to_f::calculate_fahrenheight;
use c_to_f::calculate_celcius;
//the conversions (incorrect, update later)
// C -> F (x*1.8) + 32)
// F -> C (x*(5/9)) - 32

fn main() {
    //NOTE: try and take in input as float
    //take in user choice 1 for F -> C and 2 for 200 C -> F
    let mut conversion_choice = String::new();
    println!("1) Fahrenheight -> Celcius");
    println!("2) Celcius -> Fahrenheight");
    println!("Which conversion would you like to make: ");
    io::stdin().read_line(&mut conversion_choice).expect("Not a valid input!");
    
    //turn input into integer 1 or 2
    let conversion_choice: i64 = conversion_choice.trim().parse().expect("NaN");
    
    //if 1 convert from f to c
    //if 2 convert from c to f
    if conversion_choice == 1 {
        let mut temp = String::new();
        println!("Input temperature: ");
        io::stdin().read_line(&mut temp).expect("Not a valid temperature.");
        let mut temp: i64 = temp.trim().parse().expect("NaN");
        println!("Heard: {}", temp);

        calculate_celcius(temp);

    } else if conversion_choice == 2 {
        
        let mut temp = String::new();
        println!("Input temperature: ");
        io::stdin().read_line(&mut temp).expect("Not a valid temperature.");

        let mut temp: i64 = temp.trim().parse().expect("NaN");
        println!("Heard: {}", temp);

        calculate_fahrenheight(temp);
    } else {
        println!("Input a valid number");
    }
}














