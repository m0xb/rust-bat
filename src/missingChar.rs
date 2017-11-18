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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missingChar() {
        let s = vec!["hell", "hello", 4, "myman", "my man", 2, "", "", 0, "ass", "mass", 0, "soke", "stoke", 1];
        assert_eq!(s[0], missingChar(s[1].to_string, s[2]));        
        assert_eq!(s[3], missingChar(s[4].to_string, s[5]));
        assert_eq!(s[6], missingChar(s[7].to_string, s[8]));
        assert_eq!(s[9], missingChar(s[10].to_string, s[11]));
        assert_eq!(s[12], missingChar(s[13].to_string, s[14]));
    }
}


            
