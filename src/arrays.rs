use std::mem;

pub fn run() {
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];

    arr[3]=3434;

    println!("{:?}", arr); //* :? is the debug print

    // * to get a single value from the array
    println!("{}", arr[0]);

    // * to get the length of the array
    println!("Length of the array is {}", arr.len());

    // * array allocation in stack
    println!("Array occupies {} bytes", mem::size_of_val(&arr));

    // * get the array as slice
    let slice: &[i32]=&arr[0..3];
    println!("{:?}", slice);
}

