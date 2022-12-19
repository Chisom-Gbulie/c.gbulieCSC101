//Rust program to calculate the area of a triangle given three slides

use std::io;

fn main ()
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();
    let mut input10 = String::new();
    let mut input11 = String::new();
    let mut input12 = String::new();
    let mut input13 = String::new();


//List Of Equations-----------------------------------------------------------------------------------------------------------------------
    println!("Is it true or false (True/False) you would like to perform the equation for the Area of a Trapezium?: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:bool = input1.trim().parse().expect("Not a valid number");

    println!("Is it true or false (True/False) you would like to perform the equation for the Area of a Rhombus?: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:bool = input2.trim().parse().expect("Not a valid number");

    println!("Is it true or false (True/False) you would like to perform the equation for the Area of a Parallelogram?: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:bool = input3.trim().parse().expect("Not a valid number");

    println!("Is it true or false (True/False) you would like to perform the equation for the Area of a Cube?: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    let d:bool = input4.trim().parse().expect("Not a valid number");

    println!("Is it true or false (True/False) you would like to perform the equation for the Volume of a Cylinder?: ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let e:bool = input5.trim().parse().expect("Not a valid number");


//Trapezium------------------------------------------------------------------------------------------------
    if a==true;
    {
    println!("Enter the height of Trapezium: ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let h:f32 = input6.trim().parse().expect("Not a valid number");

    println!("Enter the length of the 1st base of the trapezium: ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    let base1:f32 = input7.trim().parse().expect("Not a valid number");

    println!("Enter the length of the 2nd base of the trapezium: ");
    io::stdin().read_line(&mut input8).expect("Not a valid string");
    let base2:f32 = input8.trim().parse().expect("Not a valid number");

    let Area_of_Trapezium:f32 = (h/2) * (base1 + base2);
}
    println!("The Area of the Trapezium is: {}", Area_of_Trapezium);


//Rhombus-------------------------------------------------------------------------------------------------------
    if b==true;
    {
    println!("Enter the length of the 1st Diagonal of the Rhombus: ");
    io::stdin().read_line(&mut input9).expect("Not a valid string");
    let dia1:f32 = input9.trim().parse().expect("Not a valid number");

    println!("Enter the length of the 2nd Diagonal of the Rhombus: ");
    io::stdin().read_line(&mut input10).expect("Not a valid string");
    let dia2:f32 = input10.trim().parse().expect("Not a valid number");

    let Area_of_Rhombus:f32 = 0.5 * (dia1 * dia2);
}
    println!("The Area of the Rhombus is: {}", Area_of_Rhombus);


//Parallelogram--------------------------------------------------------------------------------------------------
    if c ==true;
    {
    println!("Enter the length of the base of the Parallelogram: ");
    io::stdin().read_line(&mut input11).expect("Not a valid string");
    let base3:f32 = input11.trim().parse().expect("Not a valid number");

    println!("Enter the length of the altititude of the Rhombus: ");
    io::stdin().read_line(&mut input12).expect("Not a valid string");
    let alt:f32 = input12.trim().parse().expect("Not a valid number");

    let Area_of_Parallelogram:f32 = base3 * alt;
}
    println!("The Area of the Parallelogram is: {}", Area_of_Parallelogram);


//Cube---------------------------------------------------------------------------------------------------------
    if d==true;
    {
    println!("Enter the length of the side of the Cube: ");
    io::stdin().read_line(&mut input13).expect("Not a valid string");
    let len:f32 = input13.trim().parse().expect("Not a valid number");

    let Area_of_cube:f32 = 6 * (len * len);
}
    println!("The Area of the Cube is: {}" Area_of_cube);


//cylinder--------------------------------------------------------------------------------------------------
    if e==true;
    {
    println!("Enter the length of the radius of the Cylinder: ");
    io::stdin().read_line(&mut input14).expect("Not a valid string");
    let rad:f32 = input14.trim().parse().expect("Not a valid number");

    println!("Enter the height of the Cylinder: ");
    io::stdin().read_line(&mut input15).expect("Not a valid string");
    let h2:f32 = input15.trim().parse().expect("Not a valid number");

    let Volume_of_Cylinder:f32 = 3.142 * (rad * rad) * h2\
}
    println!("The Volume of the Cylinder is: {}", Volume_of_Cylinder)

