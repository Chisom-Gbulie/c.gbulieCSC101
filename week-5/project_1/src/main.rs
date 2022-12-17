//Rust program to find the roots of a quadratic

use std::io;

fn main ()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value for a: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value for b: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value for c: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

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
