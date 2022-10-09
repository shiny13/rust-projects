pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

    // Cannot directly print int, must have a placeholder
    println!("Number: {}", 1);

    // Basic formatting with placeholders
    println!("{} is from {}.", "Shahnawaz", "Melbourne");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}.", 
        "Shahnawaz", "Melbourne", "code"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}.", 
        name = "John", activity = "football"
    );

    // Placeholder traits
    println!(
        "Binary: {:b} Hex: {:x} Octal: {:o}", 
        10, 10, 10
    );

    // Placeholder for debug trait
    println!(
        "{:?}", 
        (12, true, "hello")
    );

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
