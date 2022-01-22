//  * primitive str are immutable and of fixed length somewhere in the memory
// * String = growable string, heap allocated data structure

pub fn run() {
    // ! this is the first way to create a string which is immutable
    let helo = "Hello";

    // ! this is the second way to create a string which is a growable data structure
    let mut hello = String::from("Hello ");

    // * get length using this method... this method would work for either types of strings
    hello.push('W');
    // * this method would add a character to the end of the string
    hello.push_str("orld");
    // * this method would add a string to the end of the string

    // * this method would return the length of the string
    println!("Length of {}", hello.len());

    // * this method would return the capacity of the string
    println!("Capacity of {}", hello.capacity());

    // * this method would return the is empty or not
    println!("Is empty {}", hello.is_empty());

    // * this method lets you know if the string contains a particular string
    println!("Contains 'World' {}", hello.contains("World"));

    // * this method replace the string with another string
    println!("Replace 'World' with 'There' {}", hello.replace("World", "There"));

    println!("{}  {}", helo, hello);
}
