use std::fmt;

struct Beer {
    no: u32
}

impl fmt::Display for Beer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let total = phrase(self.no);
        write!(f, "{}", total)
    }
}

fn phrase(no: u32) -> String{
   let mut song = String::new();
   if no == 0 {
       let gstr = format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
       song.push_str(&gstr[..]);
   }
   else if no == 1 {
       let gstr = format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
       song.push_str(&gstr[..]);
   }
   else {
       let verb = if no -1  == 1 {"bottle"} else {"bottles"};
       let gstr = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} {} of beer on the wall.\n", no, no, no-1, verb);
       song.push_str(&gstr[..]);
   }
   return song;
}


pub fn verse(n: u32) -> String{
    let b = Beer {no: n};
    return format!("{}", b);
}

pub fn sing(n: u32, x: u32) -> String{
    let mut poem = vec![];
    for i in 0..n+1{
        if n - i < x{
            println!("{}", n-i);
            break;
        }
        poem.push(verse(n-i));
    }
    return poem.join("\n");
}