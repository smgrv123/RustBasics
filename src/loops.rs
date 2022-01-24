// * loops are meant to iterate till a condition is met

pub fn run() {
    // * Infinite loop

    let mut count: i16 = 0;
    loop {
        count += 1;
        println!("Num {}", count);

        if count == 10 {
            break;
        }
    }

    // * while loop
    let mut num: i16 = 0;
    while num <= 100 {
        if num % 15 == 0 {
            println!("FizzBuzz")
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", num)
        }
        num += 1;
    }

    // * for loop (Range)
    for num in 1..100 {
        if num % 15 == 0 {
            println!("FizzBuzz")
        } else if num % 3 == 0 {
            println!("Fizz");
        } else if num % 5 == 0 {
            println!("Buzz")
        } else {
            println!("{}", num)
        }
    }
}
