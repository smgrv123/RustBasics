// * functions are used 

pub fn run(){
    greeting("Sumrit");
    let ans:i32=add(1, 32);
    println!("{}",ans);

    let add_num=|n1:i32,n2:i32|n1+n2;
    println!("{}",add_num(5,5));
}

fn greeting(name:&str){
    println!("{}",name)
}

fn add(n1:i32,n2:i32)->i32{
    n1+n2
}

