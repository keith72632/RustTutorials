

pub fn run()
{
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //Pushing
    numbers.push(6);

    println!("{}", numbers[0]);

    println!("{}", numbers.len());

    println!("{} bytes", std::mem::size_of_val(&numbers));

    //Slices
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);

    numbers.pop();

    println!("{:?}", numbers);

    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);
    
}