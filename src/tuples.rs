pub fn run(){
    // * tuples are group of which can be of different types
    let tup:(&str,&str,i8)=("Sumrit","Delhi",20);
    println!("{} is from {} and is {}",tup.0,tup.1,tup.2);
}