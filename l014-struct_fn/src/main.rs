#[derive(Debug)]
struct Dog{
    name:String,
    height:f32,
    age:f32,
}

impl Dog{
    fn get_name(&self)->&str{
        &(self.name[..])
    }
    fn get_height(&self)->f32{
        self.height
    }
}

impl Dog{
    fn get_age(&self)->f32{
        self.age
    }
}


fn main() {
    let dog = Dog{
        name:String::from("dahuang"),
        height:1.0,
        age:2.0,
    };
    println!("dog name:{}",dog.get_name());
    println!("dog age:{}",dog.get_age());
    println!("dog height:{}",dog.get_height());

    println!("dog:{:?}",dog);
    println!("Hello, world!");
}
