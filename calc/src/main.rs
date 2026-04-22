use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter a number for a: ");
    io::stdin().read_line(&mut input).expect("Failed to read"); 
    let a: i32 = input.trim().parse().expect("Please enter a number");
    println!("Please enter a number for b: ");
    let mut input_b = String::new();
    io::stdin().read_line(&mut input_b).expect("Failed to read");
    let b: i32 = input_b.trim().parse().expect("Failed to read");
    let mut choice = String::new();
    println!("Please enter your choice: ");
    io::stdin().read_line(&mut choice).expect("Failed to read");
    let c: i32 = choice.trim().parse().expect("Please enter a valid choice");
    let x = if c == 1 {
        add(a, b)
    } else if c == 2 {
        subtract(a, b)
    } else if c == 3 {
        multiply(a, b)
    } else if c == 4 {
        divide(a, b)
    } else {
        println!("Invalid choice!");
        0
    };
    println!("Result: {}", x);
}

fn add(val1: i32, val2: i32) -> i32{
    val1 + val2
}

fn subtract(val1: i32, val2: i32) -> i32{
    val1 - val2
}

fn multiply(val1: i32, val2: i32) -> i32{
    val1 * val2
}

fn divide(val1: i32, val2: i32) -> i32 {
    if val2 == 0 {
        println!("Can't divide by zero!");
        return 0;
    }
    val1 / val2
}