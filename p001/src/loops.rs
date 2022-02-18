pub fn run()
{
    // let mut count = 0;

    //infinite loop
    // loop {
    //     count += 1;
    //     println!("Number {}", count);

    //     if count == 20 { break; }
    // }

    //While loop
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("Fizz buzz")
    //     }
    //     else if count % 3 == 0 {
    //         println!("Fizz");
    //     }
    //     else if count % 5 == 0 {
    //         println!("Buzz");
    //     }
    //     println!("Count {}", count);
    //     count+=1;
    // }

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    for x in 0..arr.len() {
        arr[x] += 3;
    }
    for x in arr {
        println!("Array element {}", x);
    }
}