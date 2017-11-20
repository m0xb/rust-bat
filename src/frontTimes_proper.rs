#[allow(non_snake_case)]
fn frontTimes(phrase: String, count: i32) -> String {
    let mut short_phrase = String::from("");
    let mut new_phrase   = String::from("");
    if phrase.len() >= 3 {
        for i in 0..3 {
            short_phrase.push(phrase.chars().nth(i).unwrap());
        }
        for _i in 0..count {
            new_phrase.push_str(&short_phrase);
        }
        return new_phrase;
    } else {
        for _i in 0..count {
            new_phrase.push_str(&phrase);
        }
        return new_phrase; 
    }
}

fn main() {
    let tests = vec![
        ("Chocolate", 2),
        ("Chocolate", 3),
        ("Abc", 3),
        ("Ab", 4),
        ("A", 4),
        ("", 4),
        ("Abc", 0)];
    for i in tests {
        println!("{}", frontTimes(i.0.to_string(), i.1));
    }
}


                            



