#[allow(non_snake_case)]
fn frontTimes(phrase: String, count: u32) -> String {
    let mut new_phrase = String::from("");
    let mut short_phrase = String::from("");
    let mut i = 0;
    let mut j = 0;
    while i < count {
        short_phrase.push(phrase.chars().nth(i as usize).unwrap());
        i += 1;
    }
    while j < count {
        new_phrase.push_str(&short_phrase);
        j += 1;
    }
    return new_phrase;
}

fn main() {
    let tests = vec![ 
        ("Chocolate", 0),
        ("Chocolate", 1),
        ("Chocolate", 2),
        ("Chocolate", 3),
        ("Chocolate", 4),
        ("Chocolate", 5),
        ("Chocolate", 6),
        ("Chocolate", 7),
        ("Chocolate", 8),
        ("Chocolate", 9)];

    for i in tests {
        println!("{}", frontTimes(i.0.to_string(), i.1));
    }
}

