#[derive(Debug)]

struct Queue<T>{
    qdata: Vec<T>,
}

impl<T> Queue<T>{
    fn new() ->Self{
        Queue{
            qdata: Vec::new()
        }
    }

    fn push(&mut self,item: T){
        self.qdata.push(item);
    }

    fn pop(&mut self) ->Option<T>{
        let len = self.qdata.len();
        if len>0{
            let t = self.qdata.remove(0);
            Some(t)
        }else{
            None
        }
    }
}

fn main() {
    let mut q = Queue::new();
    q.push(1);
    q.push(2);
    println!("q = {:?}",q);

    q.pop();
    println!("q = {:?}",q);

    q.pop();
    println!("q = {:?}",q);

    q.pop();
    println!("q = {:?}",q);
    println!("Hello, world!");
}
