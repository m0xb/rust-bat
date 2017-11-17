fn missingChar(string: String, index: u32) -> String {
    let mut m_string = String::with_capacity(string.len() - 1);
    for i in 0..(index as usize) {
        m_string.push(string.chars().nth(i).unwrap());
    }
    for i in (index as usize + 1)..string.len() {
        m_string.push(string.chars().nth(i).unwrap());
    }
    return m_string;
}


fn main() {
    let s = "hello";
    println!("{}", missingChar(s.to_string(), 1));
}


