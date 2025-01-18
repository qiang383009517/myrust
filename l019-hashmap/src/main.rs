// #[derive(Debug)]
use std::collections::HashMap;

fn main() {
    //1 create hashmap
    let mut hash = HashMap::new();
    hash.insert(String::from("red"), 10);
    hash.insert(String::from("bule"),20);
    println!("hash = {:?}",hash);

    //2 vector to hashmap
    let keys = vec![String::from("yellow"),String::from("green"),String::from("black")];
    let vals = vec![10,30,50];
    let hash1:HashMap<_,_> = keys.iter().zip(vals.iter()).collect();    

    //3 read cell
    let key = String::from("yellow");
    let val = hash1.get(&key);
    if let Some(v) = val{
        println!("key = {},val = {}",key,v);
    };
    
    //4 iter hashmap
    for (key,val) in hash1{
        println!("key:{},val:{}",key,val);
    }
    
    //5 update hashmap   
    //5.1 insert hashmap
    hash.insert(String::from("red"),30);//cover the old key:"red"
    hash.insert(String::from("red"),80);
    //5.2 insert hashmap only new key
    hash.entry(String::from("red")).or_insert(90);//try insert if key is not old
    println!("hash:{:?}",hash);

    //5.3 update old key's value
    let text = String::from("hello world wonderful world");
    let mut mp= HashMap::new();
    for word in text.split_whitespace(){
       let mut count =  mp.entry(word).or_insert(0);
       *count += 1;
    }
    println!("mp:{:?}",mp);
    println!("Hello, world!");
}
