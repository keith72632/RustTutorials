enum Movement {
    Up,
    Down,
    Left,
    Right
}

fn move_avatart(m: Movement)
{
    //Basically a switch
    match m {
        Movement::Up => println!("Up"),
        Movement::Down => println!("Down"),
        Movement::Left => println!("Left"),
        Movement::Right => println!("Right")
    }
}
pub fn run()
{
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Down;
    let avatar4 = Movement::Up;

    move_avatart(avatar1);
    move_avatart(avatar2);
    move_avatart(avatar3);
    move_avatart(avatar4);
}