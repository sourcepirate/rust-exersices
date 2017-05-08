#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
   fn new(el: T) -> Self{
       Node {
           data: el,
           next: None
       }
   }

   fn add(&mut self, el: T){
       match self.next {
           None => {
               self.next = Some(Box::new(Node{
                   data: el,
                   next: None
               }));
           },
           Some(ref mut val) => {
               val.add(el);
           }
       }
   }
}


fn main(){
    let mut a: Node<u32> = Node::new(2);
    a.add(3);
    a.add(4);
    a.add(5);
    println!("{:?}", a);

}