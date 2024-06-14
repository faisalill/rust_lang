use std::mem;

fn main() {
    let x: isize = 330034534;
    println!("Size in bytes: {}", mem::size_of_val(&x));
    println!("Size in bits: {}", mem::size_of_val(&x) * 8);

    let x: usize = 330034534;
    println!("Size in bytes: {}", mem::size_of_val(&x));
    println!("Size in bits: {}", mem::size_of_val(&x) * 8);

    println!("True and false is {}", true && false);

    println!("Tupels");
    let tup: (isize, isize) = (3, 4);
    println!("{:?}", tup);
    println!("{:#?}", tup);
    let tup: (isize, &str, usize) = (3, "hello", 38);
    let (num, name, age) = tup;
    println!("Num: {} Name: {} Age: {}", num, name, age);

    println!("Arrays And Slices");
}
