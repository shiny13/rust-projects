// Variables hold primitive data or references to data 
// Variables are immutable by default 
// Rust is a block-scoped language 

pub fn run() {
    let name = "Brad";
    let mut age = 35;
    println!("My name is {} and I am {}.", name, age);
    age += 1;
    println!("My name is {} and I am {}.", name, age);

    // Define constant, use all upper case for const
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 35);
    println!("{} is {}", my_name, my_age);
}
