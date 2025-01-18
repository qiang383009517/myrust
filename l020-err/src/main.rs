use core::panic;
use std::fs::File;
fn main() {
    //1 unreset err
    // panic!("crash and burn");

    //2 can reset err,Ruset<T,E>
    let f = File::open("hello.txt");
    let r = match f{
        Ok(file) => file,
        Err(error) => {
            panic!("no file hello.txt");
        }
    };
    println!("Hello, world!");
}
