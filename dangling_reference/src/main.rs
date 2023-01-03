fn main() {
    let reference_to_nothing = dangle();
    println!("The value of reference_to_nothing {}", reference_to_nothing);
}

fn dangle() -> String { // dangle returns a reference to a &String, solution is to return String
    let s = String::from("hello"); // s is a new String
    //&s // we return a reference to the String, s
    s // The solution to return s
} // Here, &s goes out of scope, and is dropped. Its memory goes away. Danger!
