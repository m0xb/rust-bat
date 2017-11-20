#[allow(non_snake_case)]
fn doubleX(s: String) -> bool {
    for i in 0..s.len() {
        if s.chars().nth(i).unwrap() == 'x' {
            if i + 1 < s.len() && s.chars().nth(i+1).unwrap() == 'x' {
                return true;
            }
            return false;
        }
    }
    return false;
}

fn main() {
    let tests = vec![
        "axxbb",
        "axaxax",
        "xxxxx",
        "xaxxx",
        "aaaax",
        "",
        "abc",
        "x",
        "xx",
        "xax",
        "xaxx"];
    for i in tests {
        println!("{}", doubleX(i.to_string()));
    }
}


