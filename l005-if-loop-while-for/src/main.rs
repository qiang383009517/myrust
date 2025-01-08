fn main() {
    //if 
    let y = 1;
    if y == 1{
        println!("y = {}",y);
    }
    //if else
    if y == 1{
        println!("y == 1");
    }else{
        println!("y != 1");
    }
    //if else if else
    if y == 1{
        println!("y == 1");
    }else if y == 0{
        println!("y == 0");
    }else{
        println!("y != 0 and 1");
    }

    //let if
    let cond = true;
    let y = if cond{
        5
    }else{
        6
    };
    println!("y = {}",y);
    //loop
    let mut cnt = 0;
    let ret = loop{
        if cnt >= 10{
            break cnt
        }
        cnt += 1;
        // cnt++;//err
    };
    println!("ret = {}",ret);
    //while
    let mut i = 0;
    while i<10{
        i += 1;
    }
    println!("i = {}",i);
    //for 
    let mut arr = [1,3,5,7,9];
    //for a in arr =>for a in arr.into_iter ,move a,but actruelly not
    //for a in &arr => for a in arr.iter(), ref a 
    //for a in &mut arr =>for a in arr.iter_mut(),ref and change a
    {
        for a in arr{
            println!("a = {}",a);
        }
    }
    println!("arr = {:?}",arr);
    println!("Hello, world!");
}
