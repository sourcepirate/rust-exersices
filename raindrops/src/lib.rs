pub fn raindrops(drops: u32) -> String{
    let mut sound : String = String::new();
    let mut has_factors :bool = false;
    if drops % 3 == 0{
       sound.push_str("Pling");
       has_factors = true; 
    }
    if drops % 5 == 0 {
       sound.push_str("Plang");
       has_factors = true;
    }
    if drops % 7 == 0 {
        sound.push_str("Plong");
        has_factors = true;
    }

    if !has_factors{
        let slice :String = drops.to_string();
        sound.push_str(&slice[..]);
    }

    sound
}