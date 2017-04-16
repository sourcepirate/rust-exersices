pub fn sum_of_multiples(n: u32, vs: &Vec<u32>) -> u32{
    let mut sum = 0;
    for i in 1..n {
        for k in vs.iter(){
            if i % k == 0{
                sum += i;
                break;
            }
        }
    }
    return sum;
}