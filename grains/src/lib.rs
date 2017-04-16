pub fn square(s: u32) -> u64 {
    if s <= 0 || s >= 65{
        panic!("Square must be between 1 and 64")
    }
    2u64.pow(s-1)
}

// pub fn total() -> u64 {
//   (1..65).map(|x| square(x)).fold(0, |acc, i| acc+ i)
// }

//alternative solution
pub fn total() -> u64 {
    !0 as u64  // cool short path for MAX_INT
}