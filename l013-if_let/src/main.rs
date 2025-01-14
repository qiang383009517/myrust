fn main() {

    let some_value = Some(10);
    match some_value {
        Some(3) => {println!("three")},
        _ => println!("other"),
    }
    //if let is not if ,if let is match,like if let Some(10)=some_value,it means some_value matches Some(10) or not
    if let Some(10) = some_value{
        println!("some val three");
    }else{
        println!("some val other");
    }
    
    if let a = 2{
        println!("a = {}",a);
    }else{
        println!("a != 3");
    }
    
    let b = 0;
    if b==0{
        println!("b == 0");
    }
    println!("Hello, world!");
}
