fn main() {
    println!("Hello, world!");

    // *-----------------------------------
    // *---------- Data Types -------------
    // *-----------------------------------
    // There are two types of data types in rust, scalar & compound

    // * 1: Scalar Data Types
    // -> Numbers:
    // There are three types of numbers in rust
    // signed, unsigned & floats
        // i- signed
            // there are 5 types of signed numbers, i8, i16, i32, i64, i128
            // The suffixes here (e.g. 8, 16, 32) represents the number of bits in memory that is going to be reserved for these vars. and ultimately these suffixes decide how large the variable value can be.
            // for example: i8 can store values from -2^7 - 2^7 -1
            //    similarly i32 can store values from -2^31 - 2^31 -1

        // ii- unsigned
            // similar to signed, it has 5 types too, u8, u16, u32, u64, u128
            // But its value cannot be negative. 
            // u8's value ranges from 0 - 2^8 -1

        // iii- floats
            // we have two types for floats, f32, f64

    // -> Booleans:
    let _true: bool = true;
    let _false: bool = false;

    // -> Character:
    // Character (actually char) stores a single character
    let _a_char: char = 'a';

    // * 2: Compound Data Types
    // -> Tuples:  
    // Fixed length of sequence of elements (could be of different types)
    let mut _a_tuple: (char, i32, bool) = ('a', 23, false);
    // This is how we get a value
    // print!("{}", _a_tuple.0);
    // This is how we set a value (if mutable)
    _a_tuple.2 = true;
    // Note: We can't add something to tuple after we initialize it

    // -> Arrays:
    // Fixed length of sequence of same types of elements
    let mut _an_array:[i32; 3] = [1, 2, 3];
    // This is how we get a value
    // print!("{}", _an_array[0]);
    // This is how we set a value (if mutable)
    _an_array[0] = 5;
    // Note: We can't add something to arra after we initialize it (similar to tuples)

    // *-----------------------------------
    // *---------- Console IO -------------
    // *-----------------------------------
    let mut an_input = String::new();

    println!("Enter Something: ");
    std::io::stdin().read_line(&mut an_input).expect("Coudln't read");

    println!("You Typed: {}", an_input);

}
