enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

pub fn collections_init()
{
    let _a = [1, 2, 3];
    let mut v:Vec<i32> = Vec::new();

    v.push(7);
    v.push(8);
    v.push(9);

    //Accessing out of index arrays are compile time errors, Accessing out of index vector is runtime error
    
    //put in it's own scope so because it will be dropped
    {
        let v2 = vec![1, 2, 3];
        let third = &v2[2];

        println!("Whole vector {:?} and third element {:?}", v2, third); 

        match v2.get(20) {
            Some(third) => println!("The third element is {}", third),
            None => println!("there is no third element")
        }
    }

    //Cannot have mutable and immutable reference to the same element, becuase when something in vector is changed, 
    //it's completely moved to a different spot on the heap
    let mut v3 = vec![3, 4, 5, 6];
    // let first = &v3[0];
    // v3.push(7);

    //immutable reference
    for i in &v3 {
        println!("{}", i);
    }
    //mutable reference. This can be modified
    for i in &mut v3 {
        *i*=10; // (*)==Dereference Operator 
        println!("New value {}", i);
    }

    //Enums
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];

    match &row[0] {
        SpreadsheetCell::Int(i) => println!("{}", i),
        _ => println!("Not a integer")
    }

    //Strings are stored as a collection of UTF-8 encoded bytes
    //String is a heap allocated vector, while &str is an array of characters
    let _c: char = 'u';
    let _s1 = String::new();
    let s2 = "Constants";
    let _s3 = s2.to_string();
    //String::from() passes a default value to the constructor
    let s4 = String::from("Constants");
    let mut s5 = String::from("foo");
    //push another string
    s5.push_str("Bar");
    //push another byte
    s5.push('!');
    //moving ownership of s4 into s6, then appending a reference of s5 to s6. Saves memory. Cannot use s4 directly now
    let s6: String = s4 + &s5;
    println!("{}", s6);
    let s7 = String::from("This is formatted");
    let s8 = format!("{}{}", s7, s6);
    println!("{}", s8);

    //Cannot access Strings with traditional indexing like str[0]
}