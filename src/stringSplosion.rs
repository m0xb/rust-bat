#[allow(non_snake_case)]
fn stringSplosion(s: String) -> String {
    let mut new_s = String::from("");
    let mut i = 0;
    while i < s.len() {
        for j in 0..i+1 {
            new_s.push(s.chars().nth(j).unwrap());
        }
        i += 1;
    }
    return new_s;
}

fn main() {
    let tests = vec![
        "Code",
        "abc",
        "ab",
        "x",
        "fade",
        "There",
        "Kitten",
        "Bye",
        "Good",
        "Bad"];
        for i in tests {
            println!("{}", stringSplosion(i.to_string()));
        }
    }


