fn main() {
    {
        //s is not valid in this line
        let s = "hello!";   // s is valid from this point forward
        println!("{}", s);
    } // this scope is now over, s is no longer valid

    let s = String::from("hello");
    println!("{}", s);

    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);

    {
        let s = String::from("hello"); // s is valid from this point forward
        // other operations with s
        println!("{}", s);
    } // this scope is now over, and s is no longer valid

    let mut x: u32 = 5;
    let y = x;
    println!("The value of x and y: {} {}", x, y);
    x = 6;
    // Assignment for y only copies value, not address
    println!("The value of x and y: {} {}", x, y);

    let s1 = String::from("s1 hello");
    // s1 is 'moved' into s2, s1 becomes invalid
    let s2 = s1; // A shallow copy is made, then s1 is invalidated
    // Rust copies the reference pointer for non primitive types
    // s1 will no longer be valid after assigning to s2
    // println!("s1 and s2: {} {}", s1, s2); --> error: value used here after move
    // println!("s1: {}", s1); --> error: value used here after move
    // move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait
    println!("s2: {}", s2);

    // Similar to deep copy, rust has clone to copy the heap data, not just stack data
    let s1 = String::from("hello");
    // clone operation is expensive as it copies the heap as well as stack
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Primitive types such as integers are stored entirely on the stack
    // There is no need to call clone
    // x will still be valid after the assignment operation
    // calling clone would do the same thing as integers don't have a heap
    let x = 5;
    // Integers have a 'Copy' trait, this is why older variables still stay valid
    // Rust won't let us annotate the copy trait if it has the 'Drop" trait.
    // If anything requires the call to Drop, copy cannot be applied
    let y = x;
    println!("x = {}, y = {}", x, y);

    example1();
    example2();
    example3();
}

fn example1() {
    let s = String::from("hello example1"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here
    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function,
    // but i32 is Copy, so it's okay to
    // still use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved,
    // nothing special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
}   // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.
fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn example2() {
    // gives_ownership moves its return value into s1
    let s1 = gives_ownership();
    let s2 = String::from("hello example2"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into
    // takes_and_gives_back, which also moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.
fn gives_ownership() -> String { 
    // gives_ownership will move its
    // return value into the function that calls it
    let some_string = String::from("hello example2-2"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}
// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { 
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

// Example using a tuple
fn example3() {
    let s1 = String::from("hello example3");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
