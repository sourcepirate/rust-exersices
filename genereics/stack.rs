
#[derive(Debug)]
struct Node<T>{
    data: T,
    next: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct List<T> {
    head: Option<Box<Node<T>>>
}

impl<T> List<T> {
    fn new() -> List<T>{
        List{head: None}
    }

    fn push(&mut self, elem: T) {
        let h: Box<Node<T>> = Box::new(Node{
            data: elem,
            next: self.head.take()
        });
        self.head = Some(h);
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(head) => {
                let mut uw : Node<T> = *head;
                self.head = uw.next.take();
                Some(uw.data)
            }
        }
    }
}

fn main(){
    let mut lst :List<i32> = List::new();
    lst.push(23);
    lst.push(34);
    lst.push(43);
    lst.push(45);
    println!("{:?}", lst);
    lst.pop();
    println!("{:?}", lst);
}