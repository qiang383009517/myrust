#[derive(Debug)]
struct User{
    username:String,
    email:String,
    sign_in_count:u32,
    active:bool
}

fn build_user(username:String,email:String)->User{
    User { 
        username,
        email,
        sign_in_count:1,
        active:true,
    }
}

//empty struct,you can define fn in this struct
struct empty{}

//tuple struct
#[derive(Debug)]
struct Color(i32,i32,i32);
#[derive(Debug)]
struct Point(i32,i32,i32);
fn main() {
    let mut user1:User = User{
        username:String::from("xiaoming"),
        email:String::from("xiaoming@qq.com"),
        sign_in_count:1,
        active:true,
    };

    user1.sign_in_count = 3;

    let user2 = build_user(String::from("xiaohong"), String::from("xiaohong@qq.com"));

    let user3 = User{
        username:String::from("xiaoxin"),
        email:String::from("xiaoxin@qq.com"),
        ..user1
    };
    println!("user1:{:?}",user1);
    println!("user2:{:?}",user2);
    println!("user3:{:?}",user3);

    //tuple struct
    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    println!("balck:{:?}",black);
    println!("origin:{:?}",origin);
    println!("Hello, world!");
}
