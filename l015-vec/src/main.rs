use std::vec;

fn main() {
    //1 empty vec
    let v:Vec<i32> = Vec::new();

    //2 init vec
    let v1 = vec![1,2,3,4,5];

    {
        //3 update vec
        let mut v2 = vec![3,6,9];
        v2.push(1);
        v2.push(5);
    }   //drop v2

    //4 read vec
    //4.1 index
    let v_2:&i32 = &v1[2];
    // let v_6:&i32 = &v1[6];

    //4.2 get
    let v_two = match v1.get(2){
        Some(val) => {
            println!("val:{}",val);
            val
        },
        None => {
            println!("None");
            &0
        },
    };
    let v_six = match v1.get(6){
        Some(val) => {
            println!("val:{}",val);
            val
        },
        None => {
            println!("None");
            &0
        },
    };

    //4.3 mut ref and ref
    let mut v3 = vec![5,6,7,8,9];
    let v3_1 = &v3[1];//inmutalble ref
    // v3.push(8);//change v3
    println!("v3_1:{}",v3_1);

    //5 iter vec
    //5.1 mut mode
    let v_im = vec![1,3,5];
    for i in v_im{
        println!("i = {}",i);
    }
    //5.2 const mode
    let mut v_mu = vec![1,3,5];
    for i in &mut v_mu{
        *i += 1;
        println!("i = {}",i);
    }

    //6 enum in vec
    #[derive(Debug)]
    enum Contex {
        Text(String),
        Float(f32),
        Int(i32),
    }
    let v_enum = vec![
        Contex::Text(String::from("nihao")),
        Contex::Float(2.5),
        Contex::Int(10),
    ];
    println!("v_enum:{:?}",v_enum);
    println!("Hello, world!");
}
