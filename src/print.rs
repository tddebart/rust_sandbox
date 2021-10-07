pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Formatting
    println!("Test {} and {}" , "Test2", "Test3");

    // Positional Arguments
    println!("{0} is from place and {0} likes to", "Test");

    // Named Arguments
    println!("{name} likes to play {activity}", name="My name", activity="Sports");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "Hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}