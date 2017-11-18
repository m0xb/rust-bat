pub fn delDel(string: String) -> String {
    let mut simp_string = String::with_capacity(string.len() - 3);
    if string.chars().nth(1).unwrap() == 'd' && string.chars().nth(2).unwrap() == 'e' && string.chars().nth(3).unwrap() == 'l' {
        simp_string.push(string.chars().nth(0).unwrap());
        for i in 4..string.len() {
            simp_string.push(string.chars().nth(i).unwrap());       
        }
        return simp_string;
    } else {
        return string;
    }

}

fn main() {
    let s = "hdel";
    println!("{}", delDel(s.to_string()));
}

mod tests {
    use super::*;
    #[test]

    fn test_delDel() {
        let s = vec!["hdelello", "hhdelhello", "del", "hdel", "hDel"];
        assert_eq!("hello", delDel(s[0].to_string()));
        assert_eq!("hhdelhello", delDel(s[1].to_string()));
        assert_eq!("del", delDel(s[2].to_string()));
        assert_eq!("h", delDel(s[3].to_string()));
        assert_eq!("hDel", delDel(s[4].to_string()));
    }
}

    
        
