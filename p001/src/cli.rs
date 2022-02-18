
pub fn run()
{
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    println!("Args: {:?}", args);
    println!("First arg {}", command);

    if command == "hey" {
        println!("Fuck off");
    }
}