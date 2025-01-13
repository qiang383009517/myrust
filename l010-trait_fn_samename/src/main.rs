trait Trait{
    fn foo(&mut self,x:i32);
}

struct Foo;

impl Foo {
    fn foo(&self){
        println!("Foo::foo");
    }
}

impl Trait for Foo{
    fn foo(&mut self,x:i32){
        // self.foo();//int trait for fn,compiler don't konw to call which foo(),Trait::foo or Foo::foo?
        println!("=======in trait to call foo===========");
        (&*self).foo();//explicit to call Foo::foo() in trait,mode1
        println!("{}===================={}",x,x);
        Foo::foo(&self);//explicit to call Foo::foo(),mode2
    }
}

fn main() {
    let mut a = Foo{};
    a.foo();//call Foo::foo
    // a.foo(32);// compiler only can find Foo::foo
    Trait::foo(&mut a, 32);//explicit to call trait foo(&mut self,x)
    println!("Hello, world!");
}
