// mod print;
// mod vars;
// mod types;
// mod tuples;
// mod arrays;
// mod vectors;
// mod conditional;
// mod loops;
// mod functions;
// mod pointers;
// mod structs;
// mod enums;
mod cli;

fn main() {
    // print::run();

    // //basic Formatting
    // println!("Number: {} is {}", 1, "fucked");

    // //Positional Arguments
    // println!("{0} is from {1} and likes to {2}", "keith", "hell", "fuck off");

    // //Named Arguments
    // println!("{name} likes to {activity}",
    //     name="keith", 
    //     activity="fuck off"
    // );

    // //Placeholder Traits
    // println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // //Placeholder for debug trait(argument is a tuple)
    // println!("{:?}", (12, true, "hey"));

    // //Basic Math
    // println!("10 + 10 = {}", 10 + 10);

    // vars::run(); 
    // types::run();
    cli::run();
}
