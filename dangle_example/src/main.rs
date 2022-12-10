fn main() {
    let reference_to_nothing = dangle();
    println!("s: {}", reference_to_nothing);
}

fn dangle() -> String { // dangle returns a reference to a String, returning $String will have an error
    let s = String::from("hello"); // s is a new String
    //&s // we return a reference to the String, s. It will show an error
    s // return the string s directly. This works!
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
