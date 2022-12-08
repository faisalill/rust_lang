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
    println!("Hello, world!");
}
