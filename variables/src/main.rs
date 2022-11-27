fn main() {
    // There will be a compile time error if x is not declared mutable here
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Exmaple of shadowing a variable
    // Shadow is different from immutable 
    // Shadowing only performs transformation of the value
    let y = 5;
    let y = y + 1;
    // After transformation is complete, no values can be assigned to y
    let y = y * 2;
    println!("The value of y is: {}", y);

    // Shadowing allows the change of variable type during the transformation
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
}
