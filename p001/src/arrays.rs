

pub fn run()
{
    //mut keyword isn't needed unless values will be changed
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    numbers[4] = 7;

    println!("{:?}", numbers);
    for x in numbers{
        println!("{}", x);
    }

    println!("{}", numbers[0]);

    println!("{}", numbers.len());

    println!("{} bytes", std::mem::size_of_val(&numbers));

    //Slices
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);
    
}