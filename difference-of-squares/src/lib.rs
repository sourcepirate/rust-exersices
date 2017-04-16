pub fn square_of_sum(x: u32) -> u32{
    let val = (0..x+1).fold(0, |acc, i| acc+i);
    val * val
}

pub fn sum_of_squares(x: u32) -> u32{
    (0..x+1).map(|x| x*x).fold(0, |acc, i| acc+i)
}

pub fn difference(x: u32) -> u32{
    square_of_sum(x) - sum_of_squares(x)
}