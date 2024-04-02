use std::io;

fn main() {
    variables_and_immutability();
}

fn variables_and_immutability() {
    // variables
    println!("Please enter your age: ");

    // get user's age
    let mut age = String::new();
    io::stdin()
        .read_line(&mut age)
        .expect("Something did not work");

    // convert age
    let age: u32 = age.trim().parse().expect("Did not get an integer");

    // more calculations
    println!("You are {} years old\n\n", age);

    // THESE LINES WILL FAIL because "age" is not mutable
    // age += 1;
    // println!(" Next year you will be {} years old", age);

    // constants
    const RUST_NAME: &str = "rust";
    println!("Rust name: {}", RUST_NAME);
}

fn scalar_data_types() {
    // the 4 main scalar data types

    // integers: whole numbers, numbers w/o a fractional component
    let signed_int: i8 = 127;
    let signed_int: i16 = 65_535;
    let signed_int: i32 = 2_147_483_647;
    let signed_int: i64 = 9_223_372_036_854_775_807;
    let signed_int: i128 = 1; // 3.4028237e+38
    let unsigned_int: u8 = 0;
    let unsigned_int: u16 = 0;
    let unsigned_int: u32 = 0;
    let unsigned_int: u64 = 0;
    let unsigned_int: u128 = 0;

    // integer representations
    let dec: i32 = 25; // decimal
    let hex: i32 = 0xa; // hexadecimal
    let oct: i32 = 0o7; // octal
    let bin: i32 = 0b10; // binary
    let byte: u8 = b'A'; // u8 only

    // floats
    let float: f32 = 0.0; // 3.4028237e+38
    let float: f64 = 0.1; // default

    // booleans
    let bool_true = true;
    let bool_false = false;

    // characters
    let character = 'a';
    let emoji = '✅';
}

fn compound_types() {
    // tuple: fixed size and type array
    let tup: (&str, i32) = ("Jibril", 0);
    let (name, num) = tup;
    let name = tup.0;
    let num = tup.1;

    // arrays: type and size
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    // this would result in an error
    // let array: [i32; 5] = [1, 2, 3, 4];
    // vectors can change lengths

    let array = [25, 5]; // create array with 5 spaces all set with 25s
}

fn addition_function(a: i32, b: i32) -> i32 {
    // we can return like this
    a + b
    // or
    // return a + b;
}

fn if_statements() {
    if 5 > 10 {
        //...
    } else if 5 == 10 {
        //...
    } else {
        //...
    }

    // this will not work
    // let x = if 50 == 2 / 4 { "You did it!" } else { 100_000 };
    let x = if 50 == 2 / 4 { 2 } else { 5 }; // expression values must have same types
}

fn loops() {
    // infinite loops
    let mut counter = 0;
    let result = loop {
        println!("will run forever until reaching a break statement. could be useful");
        counter += 1;

        if counter == 2 {
            break "Finished!";
        }
    };

    // classic while loop
    let mut num = 1;
    while num < 10 {
        num += 1;
    }

    // loop through a collection or iterable
    for item in 0..2 {
        // ...
    }

    let arr = [1, 2, 3, 4, 5];
    for item in arr.iter() {
        // ...
    }
}
