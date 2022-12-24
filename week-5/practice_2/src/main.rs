//Rust program to find the area of a triangle given 3 sides

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the 1st Edge of the Triangle: ");
    io::stdin().read_line(&mut input1).expect("Not a valid Input");
    let a:f32 = input1.trim().parse().expect("Not a valid Input");

    println!("Enter a 2nd Edge of a Triangle: ");
    io::stdin().read_line(&mut input2).expect("Not a valid Input");
    let b:f32 = input2.trim().parse().expect("Not a valid Input");

    println!("Enter the 3rd Edge of the Triangle: ");
    io::stdin().read_line(&mut input3).expect("Not a valid Input");
    let c:f32 = input3.trim().parse().expect("Not a valid Input");

    let s:f32 = (a + b + c) / 2.0;
    let mut area:f32 = s * (s - a) * (s - b) * (s - c);
    area = area.sqrt();

    println!("Area of a Triangle: {}", area);
}
