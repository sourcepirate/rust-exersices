/*
   Run Length encoding
*/
fn to_str_combo(n: u32) -> String{
   match n {
       1 => "".into(),
       _ => n.to_string()
   }
}

fn _sencode(s: &str) -> String {
   let mut hardstr = String::new();
   let mut count = 1;
   let mut head = s.chars().nth(0).unwrap();
   for c in s.chars().skip(1){
       if c == head {
           count += 1;
       }else {
           hardstr.push_str(&to_str_combo(count));
           hardstr.push(head);
           count = 1;
           head = c;
       }
   }
   hardstr.push_str(&to_str_combo(count));
   hardstr.push(head);
   hardstr
}

pub fn encode(s: &str) -> String {
    match s.is_empty(){
        true => s.into(),
        false => _sencode(s)
    }
}

pub fn decode(s: &str) -> String {
   let mut numerals = String::new();
   let mut hardstr = String::new();
   for c in s.chars(){
       if c.is_numeric(){
           numerals.push(c);
       }
       else {
           let count: usize = numerals.parse().unwrap_or(1);
           hardstr.push_str(&c.to_string().repeat(count));
           numerals.clear();
       }
   }
   hardstr
}