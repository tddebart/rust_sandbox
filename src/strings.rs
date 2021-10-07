pub fn run() {

    // Static string length
    let hello = "Hello";
    // Dynamic allocated string
    let mut hello2 = String::from("Hello");

    hello2 += " 2";

    if hello == hello2 {
        println!("Yes");
    }

    println!("{} {}", hello, "".is_empty());
}