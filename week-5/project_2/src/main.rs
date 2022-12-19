//Rust program to find the annual incentuves of employees based on age and experience

use std::io;

fn main ()
{
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:i32 = input1.trim().parse().expect("Not a valid age");

    println!("Enter your level of experience (with 0 indicating no experience): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience:i32 = input2.trim().parse().expect("Not a valid level");

    if age > 40
    {
        println!("Elegible to an incentive of N 1,560,000");
    }
    else if age >= 30 && age < 40
    {
        println!("Elegible to an incentive of N 1,480,000");
    }
    else if age < 28
    {
        println!("Elegible to an incentive of N 1,300,000");
    }
    else if experience == 0
    {
        println!("Elegible to an incentive of N 100,000");
    }
}
