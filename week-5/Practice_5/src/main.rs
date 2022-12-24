//Rust program to establish the height of a person and then print if person is tall, a dwarf or average height.

use std::io;

fn main()
{


    let mut input = String::new();

    println!("\nEnter Your Height In Centimetres: ");
    io::stdin().read_line(&mut input).expect("Not a valid Input");
    let Height:f32 = input.trim().parse().expect("Not a valid Input");

    if Height >= 150.0 && Height <= 170.0
    {
        println!("You are of average height person");
    }
    else if Height > 170.0 && Height <= 195.0
    {
        println!("You are Tall");
    }
    else if Height < 150.0 && Height > 100.0
    {
        println!("You are Dwarf");
    }
    else
    {
        println!("Abnormal Height");
    }
}
