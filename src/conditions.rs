pub fn run() {
    let age: u8 = 20;
    let id: bool = false;

    if age <= 20 && id {
        println!("You are young");
    } else if age > 20 && id {
        println!("You are old");
    } else {
        println!("Id please")
    }

    let short_hand: bool = if age > 21 { true } else { false };

    println!("Short hand{}", short_hand);
}
