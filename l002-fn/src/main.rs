fn other_fun(){
    println!("other fun");
}

fn other_fun1(a:u32){
    println!("other fun1 ,a = {}",a);
}

fn other_fun2(a:u32,b:u32){
    println!("other fun2 ,a={},b={}",a,b);
}

fn main() {
    other_fun();
    other_fun1(10);
    other_fun2(1,2);

    let y = {
        let x = 3;
        x+1 //return 
    };
    println!("y = {}",y);
    let y = add_two_par(2, 3);
    println!("update y =  {}",y);
    println!("Hello, world!");
}

fn add_two_par(x:u32,y:u32) ->u32{
    x+y
}
