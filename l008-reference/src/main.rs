fn main() {
    //default move 
    let s = give_ownership();
    println!("s = {}",s);
    println!("Hello, world!");
}

fn give_ownership()->String{
    let s = String::from("hello");
    s
}//move to caller

//move to param
fn take_give_ownership(s:String)->String{
    s
}

//param is reference
fn calc_len(s:&String)->usize{
    s.len()
}

//barrow 
fn change(some_string:&mut String){
    some_string.push_str(",world");
}

//reference dangle
// fn dangel()->&String{
//     let s = String::from("helloooo");
//     &s//err, s outscope ,must release it's space
// }