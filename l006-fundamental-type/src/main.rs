
// #[derive(Debug)]
use std::usize;

fn show_arr(arr:[u32;3]){
    for a in &arr{
        println!("in arr a:{}",a);
    }
}

fn main() {
    // boolean
    let is_ok:bool  = true;
    let is_false = false;
    println!("is_ok = {}",is_ok);
    println!("is_false = {}",is_false);
    // char (32byte)
    let c = 'n';
    println!("c = {}",c);
    // num: i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
    let a:i8 = 10;
    let f:f32 = 0.08;
    println!("a = {}",a);
    println!("f = {}",f);
    // usize isize
    println!("usize = {}",usize::max_value());
    // arr:[type;size]
    let arr:[u32;3] = [1,2,3];
    show_arr(arr);
    // tuple tup:(type1,type2,...)
    let t:(i32,f32,u32) = (-5,3.1415,10);
    println!("t.0={}",t.0);
    println!("t.1={}",t.1);
    println!("t.2={}",t.2);
    // println!("t={:?}",t);//err

    let t1 = (1,-2,3);
    println!("t1.0={}",t1.0);
    println!("t1.1={}",t1.1);
    println!("t1.2={}",t1.2);
    // println!("t1 = {:?}",t1);//err

    println!("Hello, world!");
}
