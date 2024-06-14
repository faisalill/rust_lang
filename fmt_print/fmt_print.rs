fn main() {
    println!("Genric Println");
    println!("HELLO WORLD");

    let hello_string = "HELLO";
    let world_string = "WORLD";

    println!("Positional Arguments");
    println!("{} {}", hello_string, world_string);

    println!("Indexed Positional Arguments");
    println!("{1} {0}", world_string, hello_string);

    println!("Named Arguments");
    println!(
        "{helloString} {worldString}",
        helloString = hello_string,
        worldString = world_string
    );

    println!("Formatting");
    let num = 100;
    println!("{number} in binary {number:b}", number = num);
    println!("{number} in octal {number:o}", number = num);
    println!("{number} in hexadecimal 0x{number:x}", number = num);
    println!("{number} in hexadecimal 0x{number:x}", number = 3334334);

    let num = 1;
    println!("Width");
    println!("{number:>5}", number = num);
    println!("{number:>width$}", number = num, width = 4);

    println!("Appending");
    println!("{number:0>5}", number = num);
    println!("{number:2>5}", number = num);

    println!("Attaching");
    println!("{number:0<5}", number = num);
    println!("{number:3<5}", number = num);
}
