// #[derive(Debug)]
fn main() {
    let s = String::from("hello world");

    let s1 = &s[2..6];
    let s2 = &s[7..];
    let s3 = &s[..3];
    let s4 = &s[0..=5];
    let s5 = &s[6..=10];
    let s6 = &s[3..];
    println!("s1:{},s2:{},s3:{},s4:{},s5:{},s6:{}",s1,s2,s3,s4,s5,s6);

    let s7 = "hello world";
    let s8 = &s7[..5];
    println!("s8 = {}",s8);

    let a = [1,3,4,7,9];
    let a1 = &a[0..3];
    println!("a1.0 = {}",&a1[0]);
    println!("a1.1 = {}",a1[1]);


    println!("Hello, world!");
}
