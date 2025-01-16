fn main() {
    let x = 123u8;
    let y:Option<&u8> = Some(&x);
    let z:Option<u8> = y.copied();

    if let Some(zz) = z{
        println!("zz = {}",zz);
    }
    println!("Hello, world!");
}
