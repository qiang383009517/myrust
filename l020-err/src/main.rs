use core::panic;
use std::{fs::File, io::ErrorKind};
fn main() {
    //1 unreset err
    // panic!("crash and burn");


    //2 can reset err, enum Ruset<T,E>{Ok(T),Err(E)}
    // let f = File::open("hello.txt");
    // let r = match f{
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("no file hello.txt");
    //     }
    // };

    // let r = match f{
    //     Ok(file)=>file,
    //     Err(error) => match error.kind(){
    //         ErrorKind::NotFound => println!("Not Found"),
    //         _ => panic!("problem opening the file,{:?}",error),
    //     },
    // };

    //3 unwrap
    // let f2 = File::open("hello.txt").unwrap();
    //4 expect
    let f3 = File::open("hello.txt").expect("not found hello.txt");
    println!("Hello, world!");
}
