// Variables hold data
// Variables are immutable by default

pub fn run() {
    let name = "Thomas";
    let mut age = 16;

    println!("My name is {} and I am {}", name, age);
    age = 38;
    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID:{}", ID);

    // Asign multiple vars
    let (my_name, my_age) = (name, age);
    println!("{} is {}", my_name, my_age);
}
