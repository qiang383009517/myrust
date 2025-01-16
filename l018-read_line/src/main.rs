use std::fs::File;
use std::io::{self,BufRead, BufReader};

fn main() -> std::result::Result<(),std::io::Error>{

    //1 
    let filename = "test.txt";
    let hd = File::open(filename)?;//? means???
    let contexts = io::BufReader::new(hd);
    for line in contexts.lines(){
        println!("line:{}",line?);
    }
    println!("-------------------------");
    //2
    let hd = File::open(filename)?;
    let mut contexts = io::BufReader::new(hd);
    let mut s = String::new();
    while contexts.read_line(&mut s).unwrap() > 0{//read_line(&mut self,&mut s)
        println!("line2:{}",s);
        s.clear();
    }

    Ok(())
}
