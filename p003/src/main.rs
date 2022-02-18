/*
What is Ownership?
    The Ownership Model is a way to manage memory. Contemporary memory management systems include Garbage Collection and
Manual Memory Managment. Garbage Collection, like used in Java, is when the interpreter handles memory managment for the
user. While safer, it means the user cannot directly access memory.
    Manual Memory Managment, like used in C/C++, is when the user has to manual allocate and deallocate memory. This is 
harder to write, and much more unsafe.
    The Ownership Model, like used in Rust, gives the user control of the memory without risk of errors, but leads to 
slower development times and a steep learning curve. Note, you can opt out of memory safety by using the unsafe keyword. 

Rust, a System Programs Language
    Rust makes certain decisions based on wether memory is allocated onto the stack, or the heap. The stack is fixed sized,
stores stack frames, which stores local functions and variables. The size of the stack, and it's contents, are determined at
compile time. Local variables within functions are dropped after function return.
    The heap contains data that can be dynamic in size, like vectors, or String. A pointer to the place in heap memory is
stored in the stack. Stack memory access is much faster than heap memory access
*/

fn main() {
    // ----- Ownership Rules -----
    // 1. Each value in Rust has a varaible trha's called it's owner.
    // 2. There can only be one owner at a time.
    // 3. WHen the owner goes out of scope, the value will be dropped.
  
    {// new scope declared
        let _s: &str = "This will be dropped"; //string literal stored on stack

        let _ss: String = String::from("This will be on the heap"); // heap memory
        // scope over, s and ss will be dropped
    }

    let _s1 = String::from("Hello");
    let _s2 = _s1; // This is called a move. Ownership is tranfers from _s1 to _s2, and _s1 is invalidated
    // println!("{}", _s1); this will not work as the previous variable is invalidated

    //How to actually clone a variable without invalidating the previous owner. This is more expensive
    let s3 = String::from("This will be cloned");
    let s4 = s3.clone();
    println!("This should work {} {}", s3, s4);
    // Stack data can be copied

    //When passing heap varaibles to a function, ownership is passed to function
    let s5 = String::from("This will be transfered");
    takes_ownership(s5);
    //can no longer use s5
    
    //Stack variables are copied
    let x: i32 = 66;
    makes_cpy(x);
    println!("x => {} after copy", x); 

    // Ownership given from function
    let s6 = give_ownership();
    println!("{}", s6); 

    // Takes ownership and moves to another variable
    let s7 = take_and_gives_back(s6);
    println!("This is now s7 {}", s7); 

    // To use a variable without taking ownership, use references. This is called borrowing
    let length = get_size(&s7);
    println!("Length of s7 {} and contents of s7 {}", length, s7);

    // references are immutable by default. To make mutable reference, do:
    let mut s8 = give_ownership();
    change_mut_variable(&mut s8);
    println!("{}", s8);

    //You can only have one mutable reference to a piece of data. You also can't have a mutable reference, AND
    //immutable reference to same data

    //Dangling References. A reference that points to invalid memory

    // Slices uses references, and do not take ownership of underlying value
    let mut sss = String::from("First Second");
    let word = first_word_index(&sss);
    sss.clear();
    println!("{}", word);
    //The problem with the above implementation is that even after sss is clear, word is still 5. Use string slice
    let sslice = &sss[0..5];
    let beginning = &sss[..5];
    println!("After slice {} {}", sslice, beginning);
}

fn takes_ownership(some_str: String)
{
    println!("{}", some_str);
}

fn makes_cpy(num: i32)
{
    println!("{} <= this is a copy", num);
}

fn give_ownership() -> String {
    let some_str = String::from("This is from give_ownership");
    some_str
}

fn take_and_gives_back(some_str: String) -> String
{
    some_str
}

fn get_size(some_str: &String) -> usize {
    let length = some_str.len();
    length
}

fn change_mut_variable(s: &mut String)
{
    s.push_str(" mutated");
}
/*

Rust will determine this function unsafe because it tries to return a value that is dropped in this scope

fn dangle() -> &String {
    let s = String::from("This will be destroyed by the end of this function");

    &s;
}
*/

fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}