/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

//* Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it.
pub fn run(){
    let a=32; // * by default this is a i32 type
    
    let f=1.2; // * by default this is a f64 type

    let v:i64=545341641051; // * we can specify the type of the variable

    println!("{}",std::i32::MAX); // * max value of i32
    println!("{}",std::i32::MIN); // * min value of i32
    println!("{}",std::i64::MAX); // * max value of i64
}