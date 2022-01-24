// ! Reference Pointers - Point to a resource in memory
pub fn run() {
    // * Primitive Array
    let arr:[i8;5]=[1,2,4,3,5];
    let arr1=arr;
    println!("{:?}",(arr,arr1));

    // * With non-primitives, if you assign another variable to a piece of data, 
    // * the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource

    let vec:Vec<i8>=vec![1,2,3,4];
    let vec1=&vec;
    println!("{:?}",(vec1,&vec));
}
