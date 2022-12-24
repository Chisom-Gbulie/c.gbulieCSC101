//Rust program to display a menu of available dishes purchasable by customers

use std::io;

fn main (){

    let _p:f32 = 3200.0;
    let _f:f32 = 3000.0;
    let _a:f32 = 2500.0;
    let _e:f32 = 2000.0;
    let _w:f32 = 2500.0;


    let mut input6 = String::new();
    println!("-Menu\np = Poundo Yam/Edinkaiko Soup -N3,200\nF = Fried Rice & Chicken -N3,000\nA = Amala & Ewedu Soup -N2,500\nE = Eba & Egusi Soup -N2,000\nW = White Rice & Stew\n[Click Enter to begin your order]");
    io::stdin().read_line(&mut input6).expect("Not a valid input");

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();

    println!("How many portions of Poundo Yam/Edinkaiko Soup would you like to order: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let P:f32 = input1.trim().parse().expect("Not a valid Input");

    println!("How many portions of Fried Rice & Chicken would you like to order: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let F:f32 = input2.trim().parse().expect("Not a valid Input");


    println!("How many portions of Amala & Ewedu Soup would you like to order: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let A:f32 = input3.trim().parse().expect("Not a valid Input");


    println!("How many portions of Eba & Egusi Soup would you like to order: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let E:f32 = input4.trim().parse().expect("Not a valid Input");


    println!("How many portions of White Rice & Stew would you like to order: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let W:f32 = input5.trim().parse().expect("Not a valid Input");

    let _total:f32 = (P * _p)+(F * _f)+(A * _a)+(E * _e)+(W * _w);

    if _total > 10000.0
    {

        let d:f32 = _total * 0.95;
        println!("Your total is {}", d);
    }
    else
    {
        println!("Your total is {}", _total);
    }
}

