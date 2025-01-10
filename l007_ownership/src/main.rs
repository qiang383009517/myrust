
fn take_ownership(some_string:String){
    println!("{}",some_string);
}// some_string droped

fn make_copy(some_int:i32){
    println!("{}",some_int);
}//i32 copy_trait

fn main() {

    //1 copy for fundamental type=>eg..bool char u8/16/32/64 i8/16/32/64 f32 tuple by fundamental type,they has trait Copy_trait,but do not has Drop_trait
    // so,they can alive to '}'
    {
        let x = 5;//x in stack
        let y = x;//y in stack
        println!("x = {}",x);
        println!("y = {}",y);
    }// x/y release
    println!("------------------------");
    //2 shallow copy=>if the type has Drop_trait,they default copy mode is shallow copy,so the ownership moved to the copyist
    {
        let s1 = String::from("hello");// in heap
        println!("s1 = {}",s1);
        let s2 = s1;//s1 ownership has moved to s2
        println!("s2 = {}",s2);
        // println!("s1 = {}",s1);
    }//s2 droped
    println!("-----------------------");
    //3 deep copy
    {
        let s1 = String::from("hello");
        println!("s1 = {}",s1);
        let s2 = s1.clone();//deep copy to s2
        println!("s2 = {}",s2);
        println!("s1 = {}",s1);
    }// s1 s2 droped

    println!("++++++++++++++++++++++++++++++");
    //fn socpe
    let s = String::from("world");
    take_ownership(s);//s move to s1
    // println!("s =  {}",s);//err s moved

    let a = 10;
    make_copy(a);//a copy to make_copy()
    println!("a = {}",a);

    println!("----------------------");
    let str1 = give_ownership();
    println!("str1 = {}",str1);

    let str2 = String::from("shijie");
    println!("str2 = {}",str2);
    let str3 = take_give_ownership(str2);//str2 moved
    // println!("str2 = {}",str2);//err
    println!("str3 = {}",str3);
}

fn give_ownership()->String{
    let s = String::from("nihao");
    s
}//return s ownership to caller

// fn give_ownership_ref()->&String{
//     let s = String::from("nihao");
//     &s
// }//return s ownership ref is err,s has released it's heap space

fn take_give_ownership(s:String)->String{
    s
}