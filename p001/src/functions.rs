pub fn run()
{
    greeting("Hello", "Keith");

    let val: i32 = add(4,6);
    println!("{}", val);

    //Closure
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str){
    println!("{}, {}", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    //no semicolon means return
    n1 + n2
}