//Tuples group togeher values of different types
//Max 12 elements

pub fn run()
{
    let person: (&str, &str, i8) = ("Keith", "horton", 44);

    println!("{} {} {}", person.0, person.1, person.2);
}