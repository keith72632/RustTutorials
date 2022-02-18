//Varibales hole primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run()
{
    //Immutable variables
    let name = "Keith";
    let age = 30;

    //Mutable 
    let mut age2 = 38;
    age2 = 40;

    println!("My name is {} and I am {} years old and {}", name, age, age2);

    //Define Constants. All uppercase, and must have type
    const ID: i32 = 001;
    println!("{}", ID);

    //Assign multiple variables
    let (my_name, my_age) = ("keith", 55);
    println!("My name {} my age {}", my_name, my_age);

}