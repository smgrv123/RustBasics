pub fn run() {
    println!("Hello, world! in print.rs");
    // Basic formatting
    println!("{}{}", "Hello", "Rust");
    // Positional arguments
    println!("{0} {1} {0}", "Hello", "Rust");
    // named arguments
    println!("{name} {lang}", name = "Rust", lang = "is awesome");
}
