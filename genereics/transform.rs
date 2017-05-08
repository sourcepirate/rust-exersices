//learning assosiated types

use std::ops::Add;

trait Transform<Input>{
     type Output;
     fn transform(&self, val: Input) -> Self::Output;
}

#[derive(Debug)]
struct Adder<T> {x: T}


impl Transform<i32> for Adder<i32> {
    type Output = i32;

    fn transform(&self, val: i32) -> i32 {
       &self.x + val  
    }
}

impl Transform<String> for Adder<String> {
   type Output = String ; 
   
   fn transform(&self, val: String) -> String {
     val + &self.x
   }
}


fn main(){
  let g :String = "abc".to_string();
  let piece :String = "my".to_string();
  let a: Adder<i32> = Adder { x: 23};
  let b: Adder<String> = Adder {x: g};
  println!("{:?}", a.transform(2));
  println!("{:?}", b.transform(piece));
}
