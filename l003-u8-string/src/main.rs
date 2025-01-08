fn main() {
    use std::fmt::Write;
    let mut str = String::new();
    let code = [177,187,202,36,165,37,62,88,
    52,55,68,];
    for a in code.iter(){
        println!("a = {}",a);
        // str.push(a);//err,u8 to char
        write!(str,"{:02x}",a);
    };
    println!("str = {}",str);
    println!("Hello, world!");
}
