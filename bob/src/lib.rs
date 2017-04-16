pub fn reply(stmt: &'static str) -> String {
    let arr: Vec<char>  = String::from(stmt).chars().collect();

    if arr.is_empty(){
        return "Fine. Be that way!".to_string();
    }

    if arr[arr.len() - 1].to_string() == "?"{
       return "Sure.".to_string();
    }
    
    if no_caps(&arr) > 1 {
       return "Whoa, chill out!".to_string();
    }

    return "Whatever.".to_string();
}

fn no_caps(stmt: &Vec<char>) -> usize {
    let result :Vec<&char> = stmt.iter().filter(|x| x.is_uppercase()).collect();
    result.len()
}