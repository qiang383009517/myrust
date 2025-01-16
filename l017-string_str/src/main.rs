fn main() {
    //1 empty string
    let mut s = String::new();
    println!("s = {}",s);
    s.push_str("string");
    println!("update s = {}",s);

    //2 from 
    let s1= String::from("String::from");
    println!("s1 = {}",s1);
    //3 str
    let s2= "from str to string".to_string();
    println!("s2 = {}",s2);
    //4 utf-8
    let s3 = String::from("你好");
    println!("s3 = {}",s3);
    
    //5 update string
    //5.1 push_str
    let mut ss = String::from("string raw");
    println!("ss = {}",ss);
    ss.push_str("push_str");
    println!("after push_str ss = {}",ss);
    let ss1 = "ss1 ***";
    ss.push_str(ss1);//ref ss1
    println!("ss1 = {}",ss1);
    println!("after ss1 ,ss={}",ss);
    //5.2 push char
    let mut sl = "sl".to_string();
    println!("sl = {}",sl);
    sl.push('l');
    println!("sl push after ,sl={}",sl);
    //5.3 string+string
    let ss2 = String::from("hello");
    let ss3 = String::from("2025");
    let ss4 = ss2 + &ss3;
    // println!("ss2 = {}",ss2);
    println!("ss3 = {}",ss3);
    println!("ss4 = {}",ss4);
    //5.4 format!()
    let s11 = String::from("hello");
    let s12 = String::from("rust");
    let s13 = String::from("new");
    let s14 = format!("{}--{}--{}",s11,s12,s13);
    let s15 = format!("{}{}{}",s11,s12,s13);
    println!("s14 = {}",s14);
    println!("s15 = {}",s15);

    //6 index string is ilegal
    let s21 = String::from("你好");
    // let s211 = s21[0];//err
    println!("s21.len = {}",s21.len());
    //7 index of str
    let s22 = String::from("你好");
    let s23 = &s22[0..3];
    println!("s23 = {}",s23);

    //8 iterator string by chars and by bytes
    let s31 = String::from("你好");
    for c in s31.chars(){
        println!("c = {}",c);
    }
    for c in s31.bytes(){
        println!("c in bytes:{}",c);
    }
    println!("Hello, world!");
}
