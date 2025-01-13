//1 like c 
enum IpKind{
    v4,
    v6,
}

// #[derive(Debug)]
struct IpAddr{
    kind:IpKind,
    addr:String,
}

//2 rust suggest
enum Ipaddress{
    v4(String),
    v6(String),
}

//3 diffrent type in enum
enum Addr{
    v4(u8,u8,u8,u8),
    v6(String),
}

//4 struct or tuple in enum
enum Message{
    Quit,
    Move{x:i32,y:i32},//struct in enum
    Write(String),//tuple in enum
    Color(i32,i32,i32)//tuple in enum
}

//4.1 fn in enum and match
impl Message{
    fn print(&self){ //self -> get the ownership,and at print end,will release the self ;&self -> don't get the ownership
        match self{
            Message::Quit => println!("Quit"),
            Message::Move{x,y} => println!("Move x:{},y:{}",x,y),
            Message::Color(r,g, b) => println!("Color {},{},{}",r,g,b),
            // Message::Write(s) => println!("Write:{}",s),
            _ => println!("Write(String)"),
        };
    }
}


fn main() {
    //1 c like enum
    let ip = IpAddr{
        kind:IpKind::v4,
        addr:String::from("127.0.0.1"),
    };
    //2 rust enumu
    let ip1 = Ipaddress::v4(String::from("127.0.0.1"));
    let ip2 = Ipaddress::v6(String::from("::1"));

    //3 
    let ip3 = Addr::v4(127,0,0,0);
    let ip4 = Addr::v6(String::from("::2"));

    //4 
    let quit = Message::Quit{};
    quit.print();
    let mov = Message::Move { x: (10), y: (20) };
    mov.print();
    let write = Message::Write(String::from("wirte something"));
    write.print();
    let color = Message::Color(125, 0, 0);
    color.print();
    println!("Hello, world!");
}
