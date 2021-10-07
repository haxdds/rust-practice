// Enums are types which have a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    match m {
        Movement::Up => println!("Up"),
        Movement::Down => println!("Down"),
        Movement::Left => println!("Left"),
        Movement::Right => println!("Right"),
    }
}

pub fn run() {

    let avatar1 = Movement::Left;

    move_avatar(avatar1);

}