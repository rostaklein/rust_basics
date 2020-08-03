use std::io;

pub fn fahrenheit() {
    let mut input = String::new();

    println!("Enter the Fahrenheit degrees:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read a line");

    let fahrenheit: f64 = input.trim().parse().expect("Input was not a number!");

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("That makes {:.1}°C", celsius);
}
