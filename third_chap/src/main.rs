use std::io;
fn main() {
    // initialising constant values
    const CONST_VARIABLE: u32 = 234;
    println!("Constant: {}", CONST_VARIABLE);

    // Shadowing => Using the let keyword to 
    // create a new variable using the name and value of another one
    // it is used to reduce the making of newvariables to convert 
    // certain variables
    let x = 5;
    let x = x + 1;
{
    let x = x+100;
    println!("The value of x inside the scope: {}", x);
}
    println!("The value of x outside the scope: {}", x);

    //Data Types
    // for a signed integer(can be negative)
    let signed_thirtytwo : i32 = 3433;
    // for a unsigned integer(can only be positive) which is based on the architecture
    let unsigned_architecture_size : usize = 34334;

    //Arithematical Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;


    //Character Types
    //In rust the char can represent more than an ascii value 

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compound Types (Can group multiple values)
    // Tuple
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    //Another way to access values of a tuple
    let x: (char, i32, u32, f32) = ('h', -23, 43, 423.435);
    println!("Element at zero index {}", x.0);

    // Arrays
    // Initialising
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // Initialize an array to contain the same value
    let a = [3; 5];
    //Accessing the values in the array
    let first = a[0];
    let second = a[1];
    println!("Zero Index: {}", first);
    println!("First Index: {}", second);
    
    //Simple array accessing program
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
    simple_function()
}

fn simple_function() {
    println!("printing another function");
}

// return data type is marked with ->
fn five() -> i32 {
    5
}
// function with multiple parameteres
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}