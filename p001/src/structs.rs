//Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8
}

//Tuple Struct
struct Color2(u8, u8, u8);

//Struct with functions
struct Person {
    first_name: String,
    last_name: String
}
impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    fn full_name(&self)->String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    fn to_tuple(self)->(String, String){
        (self.first_name, self.last_name)
    }
}

pub fn run()
{
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("{} {} {}", c.red, c.green, c.blue);

    c.red = 200;
    println!("New red value {}", c.red);

    //TUple struct
    let mut d = Color2(255, 0, 255);
    d.1 = 77;
    println!("{} {} {}", d.0, d.1, d.2);

    let mut p = Person::new("Debby", "Downer");
    println!("{} {}", p.first_name, p.last_name);
    println!("{}", p.full_name());
    p.set_last_name("Smith");
    println!("New name {}", p.full_name());
    println!("Tuple {:?}", p.to_tuple());

}