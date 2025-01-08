const MAX_POINT1:u32 = 100000;//const 

fn main() {
    const MAX_POINT:u32 = 200000;//const
    println!("MAX_POINT1 = {}",MAX_POINT1);
    println!("MAX_POINT = {}",MAX_POINT);
    // let a = 100;//unmut
    let mut a:u32 = 10;//mut
    println!("a = {}",a);
    a = 20;
    println!("update a = {}",a);
    let b = String::from("hello");
    println!("b = {}",b);
    let b = 30;//hide
    println!("update b = {}",b);
    println!("Hello, world!");
}
