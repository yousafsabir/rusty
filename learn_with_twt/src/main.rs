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
    let mut _an_array: [i32; 3] = [1, 2, 3];
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
    std::io::stdin()
        .read_line(&mut an_input)
        .expect("Coudln't read");

    println!("You Typed: {}", an_input);

    // *-----------------------------------
    // *------- Control Structures --------
    // *-----------------------------------
    // * 1: if else
    // * ----------
    let conditional_var: i32 = 5;

    if conditional_var == 6 {
        println!("conditional_var = 6");
    } else if conditional_var > 6 {
        println!("conditional_var > 6");
    } else {
        println!("conditional_var < 6");
    }

    // * conditional assignment
    let _conditional_var_1 = if conditional_var == 6 { 5 } else { 4 };

    // * 2: loops
    // * --------
    // Rust has 3 loops: for, while, loop

    // * for loop:
    for i in 0..5 {
        // will run from 0, 1, 2, 3, 4
        println!("(for loop) {i}");
    }
    for j in 0..=5 {
        // will run from 0, 1, 2, 3, 4, 5
        println!("(for loop) {j}");
    }

    // in case of arrays
    let loop_array: [char; 5] = ['H', 'e', 'l', 'l', 'o'];
    println!("(for loop) ⬇");
    for char in loop_array {
        print!("{char}")
    }
    // alternatively
    println!("(for loop) ⬇");
    for k in 0..loop_array.len() {
        print!("{}", loop_array[k]);
    }

    // * while loop:
    let mut while_var: i32 = 5;
    while while_var > 1 {
        println!("(while loop), {while_var}");
        while_var -= 1;
    }

    // * loop loop:
    loop {
        // this code runs forever unless you break it with 'break' keywod;
        break;
    }

    // if we want to return something from loop loop, its the expression followed by break keyword
    let _loop_var = loop {
        break 5 + 10;
    };

    // we can also label a loop & break it by its label. this is really handy in case of nested loops (where you want to break a specific loop)
    let mut count = 1;
    'top_loop: loop {
        loop {
            if count > 5 {
                break 'top_loop;
            } else {
                break;
            }
        }
        count = count + 1;
    }
    // *-----------------------------------
    // *------------ Functions ------------
    // *-----------------------------------
    let func_var_x = 3;
    fn add_two(a: i32, b: i32) -> i32 {
        if a == 5 {
            return a + b + 5;
        }
        a + b
    }

    let func_var_y = add_two(func_var_x, 5);

    // *-----------------------------------
    // *------------ OwnerShip ------------
    // *-----------------------------------
    /* The way rust achieves memory saftey is implementing sense of ownership into the program. Now what is memory safety:
     * Memory Safety:
     *      before we talk about what memory saftey is. we have to understand the memory types. There are two types of memory: stack & heap.
     *          stack: its a faster & organized memory in which only that data can be stored whose size (in bits) is known at programs compile time.
     *          take for example integers, the size of an i32 type var would be fixed at 32 bits.
     *          All the scalar types have fixed known size at compile time so they are stored on stack.
     *
     *          heap: its slower & less organized memory in which data of unknown size can be stored at runtime.
     *          for examples Strings in rust, we can add to Strings & it works,
     *          The way data is stored in heap is that, it stores data in heap, but pointer to that data is stored in stack.
     *
     *      How programs use memory efficiently:
     *          Each variable has a lifetime. At the start of its lifetime, memory is allocated & at the end of its lifetime, memory is returned back to ram
     *          or you can say the memory is deallocated. These are two functions that allocate & deallocate the memory. In its lifetime,
     *          its data is used by the program and when we no longer need that data, the program deallocates its memory and the variable goes out of scope(unusable).
     *          In some programming languages, you have to manually free the memory while some have Garbage Collectors to do this job for you.
     *
     *      How memory becomes unsafe:
     *          Lets say that you have two variables x & y. You assign var x some data in heap.
     *          then you assign x to var y. in this assigning, you've only copied the pointer from x to y, not the data in the heap. after sometime, when the x goes out of
     *          scope, languge calls to free its memory and the data behind the pointer gets erased. But every declared variable must has to deallocate at some time. so
     *          when y goes out of scope, language tries to free memory which has already been freed. it doesn't free the memory it thinks its freeing but something else.
     *          this is where the memory becomes unsafe.
     *
     *      How rust addresses this issue:
     *          In rust, you only have one owner of a data. when you try to assign var1, whose data is stored in heap, to var2, rust moves the value from var1 to var2 and
     *          invalidates var1. Now you can't access var1 because it has gone out of scope now.
     *          Note: this only happens with heap vars, because its expensive to actually clone a heap data from one var to another var.
     *          In case of stack vars, the value actually gets cloned to the other variable.
     *          Note: passing a heap var to a  function also takes its ownership & makes it go out of scope.
     */

    // *-----------------------------------
    // *------ Refrences & Borrowing ------
    // *-----------------------------------
    /*
     *   Since there can only be one owner of a certain data. which is the first variable its stored in. and we can't assign it to another variable without
     *   moving it, so this is a problem. But References & Borrowing are here to save us.
     *   Refrences are like pointers to the original data except they can't be invalid or dangling.
     *   Here we'll create a variable and another variable refrencing it.
     */
    let refrence_var = 5;
    let _refrence_var_2 = &refrence_var;

    /*
     *  The scope of this _refrence_var_2 is until its last used. We can have mutable refrences to mutable variables
     */
    let mut refrence_var_3 = 8;
    let _refrence_var_4 = &mut refrence_var_3;

    /*
     *  Now there are some rules regarding refrences. These are
     *  - There can be as many as you want number of immutable refrences OR only one mutable refrences at a given time
     */
    let mut refrence_var_5 = 8;

    // let _refrence_var_6 = &refrence_var_5;
    // let _refrence_var_7 = &refrence_var_5;
    // let _refrence_var_8 = &mut refrence_var_5;
    /*
     * Now the above code will cause errors, because we've immutable & mutable refrences at the same time.
     * Let's take a look at when we can have both.
     */

    let refrence_var_9 = &refrence_var_5;
    let refrence_var_10 = &refrence_var_5;

    println!("{refrence_var_9} and {refrence_var_10}"); // * here refrence_var_9 & refrence_var_10 lifetime ends, so mutable refrence in the next line is valid

    let _refrence_var_11 = &mut refrence_var_5;
    /*
     * Above code is a valid code
     */
}
