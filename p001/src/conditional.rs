pub fn run()
{
    let mut age = 22;
    let flag: bool = true;

    if age >= 21 && flag { 
        println!("Yes");
    }
    else if age == 20 {
        println!("Fuck");
    }
    else {
        println!("Blow me");
    }

    //ternary
    let of_age: bool = if age >= 21 {true} else {false};
    println!("is of age {}", of_age);
    age |= 1000;
    println!("{} after bitwise", age);
 
}