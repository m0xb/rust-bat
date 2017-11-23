fn helloName(s: String) -> String {
    let hello = String::from("Hello ");
    return hello + &s + "!";
}

fn makeOutWord(out: String, word: String) -> String {
    let mut new_s = String::from("");
    new_s += &out[0..2].to_string();
    new_s += &word.to_string();
    new_s += &out[2..4].to_string();
    return new_s;
}

fn firstHalf(s: String) -> String {
    return s[0..(s.len()/2)].to_string();
}

fn nonStart(a: String, b: String) -> String {
    let mut s = String::from("");
    s += &a[1..a.len()].to_string();
    s += &b[1..b.len()].to_string();
    return s;
}

fn theEnd(s: String, b: bool) -> String {
    let mut new_s = String::from("");
    if b == true {
        new_s.push(s.chars().nth(0).unwrap());
    } else {
        new_s.push(s.chars().nth(s.len() - 1).unwrap());
    }
    return new_s;
}

fn endsLy(s: String) -> bool {
    if s.len() > 1 && &s[(s.len() - 2)..(s.len())] == "ly" {
        return true;
    }
    return false;
}

fn middleThree(s: String) -> String {
    let mut new_s = String::with_capacity(3);
    new_s += &s[(s.len()/2 - 1)..(s.len()/2 + 2)];
    return new_s;
}

fn lastChars(a: String, b: String) -> String {
    let mut new_s = String::with_capacity(2);
    if &a == "" {
    new_s.push('@');
    } else {
        new_s.push(a.chars().nth(0).unwrap());
    }
    if &b == "" {
        new_s.push('@');
    } else {
        new_s.push(b.chars().nth(b.len() - 1).unwrap());    
    }
    return new_s;
}

fn seeColor(s: String) -> String {
    if s.len() > 2 && &s[0..3] == "red" {
        return "red".to_string();
    }
    if s.len() > 3 && &s[0..4] == "blue" {
        return "blue".to_string();
    }
    else {
        return "".to_string();
    }
}

fn extraFront(s: String) -> String {
    let mut new_s = String::with_capacity(6);
    if s.len() == 1 {
        for i in 0..4 {
            new_s.push_str(&s.to_string());
        }
        return new_s;
    } else if s.len() > 1 {
        for i in 0..4 {
            new_s += &s[0..2];
        }
        return new_s
    } else {
        return s;
    }
}

fn startWord(s: String, word: String) -> String {
    let mut new_s = String::from("");
    if s.len() >= word.len() && (word.len() < 2 || &s[1..word.len()] == &word[1..]) {
        new_s.push(s.chars().nth(0).unwrap());
        new_s += &word[1..word.len()].to_string();
    }
    return new_s;
}

fn makeAbba(a: String, b: String) -> String {
    let mut abba = String::from("");
    abba += &a.to_string();
    abba += &b.to_string();
    abba += &b.to_string();
    abba += &a.to_string();
    return abba;
}

fn extraEnd(s: String) -> String {
    let mut new_s = String::with_capacity(6);
    for i in 0..3 {
        new_s += &s[(s.len() - 2)..s.len()];
    }
    return new_s;
}

fn withoutEnd(mut s: String) -> String {
    s.remove(0);
    s.pop();
    return s;
}

fn left2(mut s: String) -> String {
    s += &s[0..2].to_string();
    s.remove(0);
    s.remove(0);
    return s;
}

fn withouEnd2(mut s: String) -> String {
    if s.len() > 0 {
        s.remove(0);
    }
    s.pop();
    return s;
}

fn nTwice(s: String, n: i32) -> String {
    let mut new_s = String::from("");
    new_s.push_str(&s[0..(n as usize)]);
    new_s.push_str(&s[(s.len() - (n as usize))..s.len()]);
    return new_s;
}

fn hasBad(s: String) -> bool {
    if s.len() > 2 && &s[0..3] == "bad" || s.len() > 3 && &s[1..4] == "bad" {
        return true;
    }
    return false;
}

fn conCat(a: String, b: String) -> String {
    let mut new_s = String::from("");
    new_s.push_str(&a.to_string());
    new_s.push_str(&b.to_string());
    if a.len() > 0 && b.len() > 0 && &a[(a.len()-1)..(a.len())] == &b[0..1] {
        new_s.remove(a.len() - 1);
        return new_s;
    }
    return new_s;
}

fn frontAgain(s: String) -> bool {
    if s.len() > 1 && &s[0..2] == &s[(s.len() - 2)..(s.len())] {
        return true;
    }
    return false;
}

fn without2(mut s: String) -> String {
    if s.len() > 1 && &s[0..2] == &s[(s.len() - 2)..(s.len())] {
        s.remove(0);
        s.remove(0);
    }
    return s;
}

fn withoutX(mut s: String) -> String {
    if s.len() > 0 {
        if s.chars().nth(0).unwrap() == 'x' {
            s.remove(0);
        } 
        if s.len() > 0 && s.chars().nth(s.len() - 1).unwrap() == 'x' {
            s.pop();
        }
    }
    return s;
}

fn main() {
    let helloName_tests = vec![
        "Bob",
        "Alice",
        "X",
        "Dolly",
        "Alpha",
        "Omega",
        "Goodbye",
        "ho ho ho",
        "xyz!",
        "Hello"];

    for i in helloName_tests {
        println!("helloName: {}", helloName(i.to_string()));
    }

    println!("\n");

    let makeOutWord_tests = vec![
        ("<<>>", "Yay"),
        ("<<>>", "WooHoo"),
        ("[[]]", "word"),
        ("HHoo", "Hello"),
        ("abyz", "YAY")];
    for i in makeOutWord_tests {
        println!("makeOutWord: {}", makeOutWord(i.0.to_string(), i.1.to_string()));
    }
    
    println!("\n");

    let firstHalf_tests = vec![
        "WooHoo",
        "HelloThere",
        "abcdef",
        "ab",
        "",
        "0123456789",
        "kitten"];
    for i in firstHalf_tests {
        println!("firstHalf: {}", firstHalf(i.to_string()));
    }
    
    println!("\n");

    let nonStart_tests = vec![
        ("Hello", "There"),
        ("java", "code"),
        ("shotl", "java"),
        ("ab", "xy"),
        ("ab", "x"),
        ("x", "ac"),
        ("a", "x"),
        ("kit", "kat"),
        ("mart", "dart")];
    for i in nonStart_tests {
        println!("nonStart: {}", nonStart(i.0.to_string(), i.1.to_string()));        
    }
    
    println!("\n");

    let theEnd_tests = vec![
        ("Hello", true),
        ("Hello", false),
        ("oh", true),
        ("oh", false),
        ("x", true),
        ("x", false),
        ("java", true),
        ("chocolate", false),
        ("1234", true),
        ("code", false)];
    for i in theEnd_tests {
        println!("theEnd: {}", theEnd(i.0.to_string(), i.1));
    }
    
    println!("\n");

    let endsLy_tests = vec![
        "oddly",
        "y",
        "oddy",
        "oddl",
        "olydd",
        "ly",
        "",
        "falsey",
        "evenely"];
    for i in endsLy_tests {
        println!("endsLy: {}", endsLy(i.to_string()));
    }
    
    println!("\n");

    let middleThree_tests = vec![
        "Candy",
        "and",
        "solving",
        "Hi yet Hi",
        "java yet java",
        "Chocolate",
        "XabcxyzabcX"];
    for i in middleThree_tests {
        println!("middleThree: {}", middleThree(i.to_string()));
    }
    
    println!("\n");

    let lastChars_test = vec![
        ("last", "chars"),
        ("yo", "java"),
        ("hi", ""),
        ("", "hello"),
        ("", ""),
        ("kitten", "hi"),
        ("k", "zip"),
        ("kitten", ""),
        ("kitten", "zip")];
    for i in lastChars_test {
        println!("lastChars: {}", lastChars(i.0.to_string(), i.1.to_string()));
    }
    
    println!("\n");

    let seeColor_test = vec![
        "redxx",
        "xxred",
        "blueTimes",
        "NoColor",
        "red",
        "re",
        "blu",
        "blue",
        "a",
        "",
        "xyzred"];
    for i in seeColor_test {
        println!("seeColor: {}", seeColor(i.to_string()));
    }

    println!("\n");

    let extraFront_tests = vec![
        "Hello",
        "ab",
        "H",
        "",
        "Candy",
        "Code"];
    for i in extraFront_tests {
        println!("extraFront: {}", extraFront(i.to_string()));
    }

    println!("\n");

    let startWord_test = vec![
        ("hippo", "hi"),
        ("hippo", "xip"),
        ("hippo", "i"),
        ("hippo", "ix"),
        ("h", "ix"),
        ("", "i"),
        ("hip", "zi"),
        ("hip", "zip"),
        ("hip", "zig"),
        ("h", "z"),
        ("hippo", "xippo"),
        ("hippo", "xyz"),
        ("hippo", "hip"),
        ("kitten", "cit"),
        ("kit", "cit")];    
    for i in startWord_test {
        println!("startWord: {}", startWord(i.0.to_string(), i.1.to_string()));
    }
    
    println!("\n");

    let makeAbba_tests = vec![
        ("Hi", "Bye"),
        ("Yo", "Alice"),
        ("What", "Up"),
        ("aaa", "bbb"),
        ("x", "y"),
        ("x", ""),
        ("", "y"),
        ("Bo", "Ya"),
        ("Ya", "Ya")];
    for i in makeAbba_tests {
        println!("makeAbba: {}", makeAbba(i.0.to_string(), i.1.to_string()));
    }
    
    println!("\n");

    let extraEnd_tests = vec![
        "Hello",
        "ab",
        "Hi",
        "Candy",
        "Code"];
    for i in extraEnd_tests {
        println!("extraEnd: {}", extraEnd(i.to_string()));
    }
    
    println!("\n");

    let withoutEnd_tests = vec![
        "Hello",
        "java",
        "code",
        "ab",
        "Chocolate!",
        "kitten",
        "woohoo"];
    for i in withoutEnd_tests {
        println!("withoutEnd: {}", withoutEnd(i.to_string()));
    }
    
    println!("\n");

    let left2_tests = vec![
        "Hello",
        "java",
        "Hi",
        "code",
        "cat",
        "12345",
        "Chocolate",
        "bricks"];
    for i in left2_tests {
        println!("left2: {}", left2(i.to_string()));
    }

        println!("\n");

    let withoEnd2_tests = vec![
        "Hello",
        "abc",
        "ab",
        "a",
        "",
        "coldy",
        "java code"];
    for i in withoEnd2_tests {
        println!("withouEnd2: {}", withouEnd2(i.to_string()));
    }

    println!("\n");

    let nTwice_tests = vec![
        ("Hello", 2),
        ("Chocolate", 3),
        ("Chocolate", 1),
        ("Chocolate", 0),
        ("Hello", 4),
        ("Code", 4),
        ("Code", 4)];
    for i in nTwice_tests {
        println!("nTwice: {}", nTwice(i.0.to_string(), i.1));
    }

    println!("\n");

    let hasBad_tests = vec![
        "badxx",
        "xbadxx",
        "xxbadxx",
        "code",
        "bad",
        "ba",
        "xba",
        "xbad",
        "",
        "badyy"];
    for i in hasBad_tests {
        println!("hasBad: {}", hasBad(i.to_string()));
    }
    
    println!("\n");

    let conCat_tests = vec![
        ("abc", "cat"),
        ("dog", "cat"),
        ("abc", ""),
        ("pig", "g"),
        ("pig", "doggy")];
    for i in conCat_tests {
        println!("conCat: {}", conCat(i.0.to_string(), i.1.to_string()));
    }

    println!("\n");

    let frontAgain_tests = vec![
        "edited",
        "edit",
        "ed",
        "jj",
        "jjj",
        "jjjj",
        "jjjk",
        "x",
        "",
        "java",
        "javaja"];
    for i in frontAgain_tests {
        println!("frontAgain: {}", frontAgain(i.to_string()));
    }

    println!("\n");

    let without2_tests = vec![
        "HelloHe",
        "HelloHi",
        "Hi",
        "Chocolate",
        "xxx",
        "xx",
        "x",
        "",
        "Fruits"];
    for i in without2_tests {
        println!("without2: {}", without2(i.to_string()));
    }
    
    println!("\n");

    let withoutX_tests = vec![
        "xHix",
        "xHi",
        "Hxix",
        "Hi",
        "xxHi",
        "Hix",
        "xaxbx",
        "xx",
        "x",
        "",
        "Hello",
        "Hexllo"];
    for i in withoutX_tests {
        println!("withoutX: {}", withoutX(i.to_string()));
    }


}

