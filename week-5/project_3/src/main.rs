//Rust program to find the roots of a quadratic

use std::io;

fn main ()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("Poundo Yam/Edinkaiko Soup: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Fried Rice & Chicken: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");

    println!("Amala & Ewedu Soup: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");

    println!("Eba & Egusi Soup: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");

    println!("White Rice & Stew: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");


    let d:f32 = (a * a) - 4.0 * a * c;

    if d >= 0.0
    {
        println!("Presence of two distinct roots");
    }
    else if d == 0.0
    {
        println!("Presence of one distinct roots");
    }
    else if d < 0.0
    {
        println!("Presence of no real roots");
    }
}
