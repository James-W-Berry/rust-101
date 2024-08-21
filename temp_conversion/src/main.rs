use std::io;

fn main() {
    let mut temp: String = String::new();
    let mut unit: String = String::new();

    println!("enter temperature");
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");
    let temp: i32 = temp.trim().parse().expect("Failed to read line");

    println!("enter unit - f or c");
    io::stdin()
        .read_line(&mut unit)
        .expect("Failed to read line");
    let unit: char = unit.trim().parse().expect("Failed to read line");
    
    if unit == 'c' {
        let fahrenheit = convert_celsius_to_fahrenheit(temp);
        println!("{temp}{unit} is {fahrenheit}f");
    } else {
        let celsius = convert_fahrenheit_to_celsius(temp);
        println!("{temp}{unit} is {celsius}c");
    }
 }

fn convert_celsius_to_fahrenheit(temp: i32) -> i32 {
    temp * 9 / 5 + 32
}

fn convert_fahrenheit_to_celsius(temp: i32) -> i32 {
    (temp - 32) * 5 / 9
}