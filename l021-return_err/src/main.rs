use std::io;
use std::io::Read;
use std::fs::File;

fn read_from_file() -> Result<String,io::Error>{
    let f = File::open("hello.txt");
    let mut f = match f{
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {//read hello.txt
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    let ret = read_from_file();
    match ret{
        Ok(s)=>println!("{}",s),
        Err(err) => println!("{}",err),
    }
    println!("Hello, world!");
}
