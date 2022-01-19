pub fn run() {
    let name = "Sumrit";
    // ! variables are immutable by default,
    // ! but can be changed by using mut keyword
    let mut age = 20;
    // ! if we dont use the initial value of the variable, it would throw a warning
    println!("My name is {} and I am {}", name, age);
    age = 21;
    println!("My name is {} and I am {}", name, age);

    // ! define constants by using const
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // ! assign multiple variables at once
    let (my_name, my_age) = ("Sumrit", 20);
    println!("{} is {}", my_name, my_age);
}
