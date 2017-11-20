#[allow(non_snake_case)]
fn stringYak(s: String) -> String {
    let mut new_s = String::from("");
    let mut i = 0;
    while i < s.len() {
        if i+2 < s.len() && s.chars().nth(i).unwrap() == 'y' && s.chars().nth(i+2).unwrap() == 'k' {
            i += 3;
            continue;
        }
        new_s.push(s.chars().nth(i).unwrap()); 
        i += 1;
    }
    return new_s;    
}

fn main() {
    let my_vec = vec!["yakpak", "pakyak", "yak123ya", "yak", "yakxxxyak", "HiyakHi", "xxxyakyyyakzzz"];
    for i in my_vec {
        println!("{}", stringYak(i.to_string()));
    }
}


