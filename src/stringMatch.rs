#[allow(non_snake_case)]
fn stringMatch(s1: String, s2: String) -> i32 {
    let mut i = 0;
    let s1_len = s1.len();
    let s2_len = s2.len();
    let mut count = 0;
    while i < s1_len && i < s2_len {
        if i + 1 < s1_len && i + 1 < s2_len && (s1.chars().nth(i as usize).unwrap() == s2.chars().nth(i as usize).unwrap() && s1.chars().nth(i + 1 as usize).unwrap() == s2.chars().nth(i + 1 as usize).unwrap()) {
            count += 1;
        }
        i += 1;
    }
    return count;
}

fn main() {
    let tests = vec![
        ("xxcaazz","xxbaaz"),
        ("abc","abc"),
        ("abc","axc"),
        ("hello","he"),
        ("he","hello"),
        ("h","hello"),
        ("","hello"),
        ("aabbccdd","abbbxd"),
        ("aaxxaaxx","iaxxai"),
        ("iaxxai","aaxxaaxx")];
    for i in tests {
        println!("{}", stringMatch(i.0.to_string(), i.1.to_string()));
    }
}

