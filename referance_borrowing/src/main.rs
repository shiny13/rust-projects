fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); 

    // Mutual reference
    let mut s = String::from("hello");
    change(&mut s);

    // You can have only one mutual reference in a score for a variable
    // The following code will fail
    //let mut s = String::from("hello");
    //let r1 = &mut s; 
    //let r2 = &mut s;

    // The following scenario is allowed as different scope was created
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no // problems.
    let r2 = &mut s;

    // Rust does not allow you to combine mutable and immutable references
    // The following code will fail
    //let mut s = String::from("hello");
    //let r1 = &s; // no problem
    //let r2 = &s; // no problem
    //let r3 = &mut s; // BIG PROBLEM

}

fn calculate_length(s: &String) -> usize { 
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of 
  // what it refers to, nothing happens.
  // when referencing a variable, it is immutable by default and cannot be changed

fn change(some_string: &mut String) { 
    some_string.push_str(", world");
}
