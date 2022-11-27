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

    // Example of a tuple
    let tup = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("The value of a, b, and c are: {} {} {}", a, b, c);

    let x_tup: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x_tup.0;
    let _six_point_four = x_tup.1;
    let _one = x_tup.2;

    // Example of array
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let _second = a[1];

    another_function(first);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);

    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    // Loops example, bad one
    loop {
        println!("A bad loop example");
        break;
    }

    // Loops example with while loop and a condition
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }

    // Similar but much faster for loop
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Looping a range, example of rev()
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
