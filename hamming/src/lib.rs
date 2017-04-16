pub fn hamming_distance(s1: &'static str, s2: &'static str) -> Result<u32, &'static str> {
    let arr1 :Vec<char> = String::from(s1).chars().collect();
    let arr2 :Vec<char> = String::from(s2).chars().collect();
    if arr1.len() != arr2.len(){
        return Err("No array");
    }
    let mut ham = 0;
    let leng = arr1.len();
    for i in 0..leng{
        if arr1[i] != arr2[i]{
            ham += 1;
        }
    }
    return Ok(ham);
}