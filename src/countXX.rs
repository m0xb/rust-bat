#[allow(non_snake_case)]
fn countXX(s: String) -> i32 {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if i + 1 < s.len() && s.chars().nth(i).unwrap() == 'x' && s.chars().nth(i+1).unwrap() == 'x' {
            count += 1;
        }
        i += 1;
    }
    return count;
}

fn main() {
    let tests = vec![
        "abcxx",
        "xxx",
        "xxxx",
        "abc",
        "Hello there",
        "Hexxo thxxe",
        "",
        "kittens",
        "Kittensxxx"];
    for i in tests {
        println!("{}", countXX(i.to_string()));
    }
}



