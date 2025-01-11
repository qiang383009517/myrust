fn main() {
    //1 default move 
    let s = give_ownership();
    println!("s = {}",s);
    let s1= String::from("you are");
    let s2 = take_give_ownership(s1);
    // println!("s1:{}",s1);//s1 moved
    println!("s2:{}",s2);

    //2 param ref
    let s3 = String::from("hero");
    let l3 = calc_len(&s3);
    println!("s3 = {}",s3);
    println!("l3 = {}",l3);

    //3 borrow and change
    let mut s4 = String::from("in the ");
    change(&mut s4);
    println!("s4 = {}",s4);

    //4 no ref and mut ref with a var in same time
    let mut ss = String::from("mamamiya");
    let ssr1 = &ss;
    let ssr2 = &ss;
    // let mut ssr3 = &mut ss; //ref and mutref are in same time 
    println!("ssr1:{},ssr2:{}",ssr1,ssr2);
    let mut ssr3 = &mut ss;//ok
    println!("ssr3:{}",ssr3);

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

//barrow (you can change the refer)
fn change(some_string:&mut String){
    some_string.push_str(",world");
}

//reference dangle
// fn dangel()->&String{
//     let s = String::from("helloooo");
//     &s//err, s outscope ,must release it's space
// }