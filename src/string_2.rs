pub fn double_char(s: String) -> String {
    let mut doubled = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        doubled.push(c);
        doubled.push(c);
    }
    return doubled;
}
