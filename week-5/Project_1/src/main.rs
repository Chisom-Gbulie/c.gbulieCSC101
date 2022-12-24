//Program to nature of the input roots of a quadratic equation through it's discriminant

use std::io;

fn main()
{

    let mut input1 = String::new();
    println!("Enter the value of the 1st root: ");
    io::stdin().read_line(&mut input1).expect("Not a valid Input");
    let a:f32 = input1.trim().parse().expect("Not a valid Input");

    let mut input2 = String::new();
    println!("Enter the value of the 2nd root: ");
    io::stdin().read_line(&mut input2).expect("Not a valid Input");
    let b:f32 = input2.trim().parse().expect("Not a valid Input");

    let mut input3 = String::new();
    println!("Enter the value of the 3rd root: ");
    io::stdin().read_line(&mut input3).expect("Not a valid Input");
    let c:f32 = input3.trim().parse().expect("Not a valid Input");

    let d:f32 = (b * b) - 4.0 * (a * c);

    if d == 0.0
    {
        println!("The Quadractic Equation has only 1 real root");
    }
    else if d > 0.0
    {
        println!("The Quadractic Equation has 2 distinct roots");
    }
    else if d < 0.0
    {
        println!("The Quadractic Equation has no real roots");
    }
}






