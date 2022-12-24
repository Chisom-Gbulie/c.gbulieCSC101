//Rust program to findn the area of a triangle for given base and height

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter Base: ");
    io::stdin().read_line(&mut input1).expect("Not a valid Input");
    let Base:f32 = input1.trim().parse().expect("Not a valid Input");

    println!("Enter Height: ");
    io::stdin().read_line(&mut input2).expect("Not a valid Input");
    let Height:f32 = input2.trim().parse().expect("Not a valid Input");

    if Base > 0.0 {
        let area:f32 = (Base * Height) / 2.0;
        println!("Area of a Traingle: {}", area);
    }
}
