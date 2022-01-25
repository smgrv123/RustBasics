// * they are used to create custom data types

// * traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// *tuple struct
struct Tuple(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // !Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // ! contruct fullname
    // * self is used to refer to the struct itself
    fn full_name(&self)->String{
        format!("{} {}",self.first_name,self.last_name)
    }

    fn set_last_name(&mut self,last:&str){
        self.last_name=last.to_string();
    }

    fn to_tuple(self)->(String,String){
        (self.first_name,self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.red=200;

    // println!("{} {} {}", c.blue, c.green, c.red);

    // let mut x=Tuple(255,0,0);

    // println!("{} {} {}", x.0,x.1,x.2);

    // x.2=10;
    // println!("{} {} {}", x.0,x.1,x.2);

    let mut p = Person::new("Jon", "Doe");
    p.set_last_name("Mary");
    println!("Person Name {} ", p.full_name());
    println!("{:?}",p.to_tuple());
}
