// * enums are def value types

enum Movement {
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m:Movement){
    match m{
        Movement::Down=>println!("down"),
        Movement::Up=>println!("up"),
        Movement::Left=>println!("left"),
        Movement::Right=>println!("right")
    }
}

pub fn run() {
    let a1=Movement::Up;
    let a2=Movement::Down;
    let a3=Movement::Right;
    let a4=Movement::Left;

    move_avatar(a1);
    move_avatar(a2);
    move_avatar(a3);
    move_avatar(a4);
}
