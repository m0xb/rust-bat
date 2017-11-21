#[allow(non_snake_case)]
fn sleepIn(b1: bool, b2: bool) -> bool {
    if b2 == true {
        return true;
    }
    if b1 == false {
        return true;
    }
    return false;
}

fn parrotTrouble(b1: bool, int: i32) -> bool {
    if b1 == true && (int < 7 || int > 20) {
        return true;
    }
    return false;
}

fn makes10(a: i32, b: i32) -> bool {
    if (a == 10 || b == 10) || a + b == 10 {
        return true;
    }
    return false;
}

fn nearHundred(n: i32) -> bool {
    if (n > 89 && n < 111) || (n > 189 && n < 211) {
        return true;
    }
    return false;
}

fn posNeg(a: i32, b: i32, negative: bool) -> bool {
    if negative == false && (a < 0 && b >= 0) {
        return true;
    } else if negative == false && (a >= 0 && b < 0) {
        return true;
    } else if negative == true && a < 0 && b < 0 {
        return true;
    } else {
        return false;
    }
}

fn frontBack(s: String) -> String {
    let mut new_s = String::from(""); 
    if s.len() > 2 {
        new_s.push(s.chars().nth(s.len()-1).unwrap());
        for i in 1..s.len()-1 {
            new_s.push(s.chars().nth(i).unwrap());
        }
        new_s.push(s.chars().nth(0).unwrap());
        return new_s;
    } else if s.len() == 2 {
        new_s.push(s.chars().nth(1).unwrap());
        new_s.push(s.chars().nth(0).unwrap());
        return new_s;
    } else if s.len() == 1 {
        new_s.push(s.chars().nth(0).unwrap());
        return new_s;
    } else {
        return new_s;
    }
}

fn front3(s: String) -> String {
    let mut new_s = String::from("");
    let s_len = s.len();
    if s_len > 2 {
        let mut i = 0;
        while i < 3 {
            new_s.push(s.chars().nth(0).unwrap());
            new_s.push(s.chars().nth(1).unwrap());
            new_s.push(s.chars().nth(2).unwrap());
            i += 1;
        }
        return new_s;
    } else if s_len <= 2 {
        let mut i = 0;
        while i < 3 {
            if s_len == 1 {
                new_s.push(s.chars().nth(0).unwrap());
            }
            if s_len == 2 {
                new_s.push(s.chars().nth(0).unwrap());
                new_s.push(s.chars().nth(1).unwrap());
            }
        i += 1;
        }
        return new_s
    } else {
        return new_s;
    }

}

fn backAround(s: String) -> String {
    let mut new_s = String::with_capacity(s.len() + 2);
    let s_len = s.len();
    new_s.push(s.chars().nth(s_len - 1).unwrap());
    for i in 0..s_len {
        new_s.push(s.chars().nth(i).unwrap());
    } 
    new_s.push(s.chars().nth(s_len - 1).unwrap());
    return new_s;
}

fn or35(n: i32) -> bool {
    if n % 3 == 0 || n % 5 == 0 {
        return true;
    }
    return false;
}

fn front22(s: String) -> String {
    let s_len = s.len();
    let mut s_new = String::with_capacity(4);
    if s_len > 2 {
        let mut s_sub = String::with_capacity(2);
        s_sub.push(s.chars().nth(0).unwrap());
        s_sub.push(s.chars().nth(1).unwrap());
        s_new.push_str(&s_sub);
        s_new.push_str(&s);
        s_new.push_str(&s_sub);
        return s_new;
    } else if s_len <= 2 && s_len > 0 {
        s_new.push_str(&s);
        s_new.push_str(&s);
        s_new.push_str(&s);
        return s_new;    
    } else {
        return s_new;
    }

    
}

    



fn main() {


    //sleepIn
    let sleepIn_tests = vec![
        (false, false),
        (true, false),
        (false, true),
        (true, true)];
    for i in sleepIn_tests {
        println!("sleepIn: {}", sleepIn(i.0, i.1));
    }
    println!("\n");

    let parrotTrouble_tests = vec![
        (true, 6),
        (true, 7),
        (false, 6),
        (true, 21),
        (false, 21),
        (false, 20),
        (true, 24),
        (false, 23),
        (true, 20),
        (false, 12)];
    for i in parrotTrouble_tests {
        println!("parrotTrouble: {}", parrotTrouble(i.0, i.1));
    }
    
    println!("\n");

    let makes10_tests = vec![
        (9, 10),
        (9, 9),
        (1, 9),
        (10, 1),
        (10, 10),
        (8, 2),
        (8, 3),
        (10, 42),
        (12, -2)];
    for i in makes10_tests {
        println!("makes10: {}", makes10(i.0, i.1));
    }

    println!("\n");

    let nearHundred_tests = vec![
        93,
        90,
        89,
        110,
        111,
        121,
        -101,
        -209,
        190,
        209,
        0,
        5,
        -50,
        191,
        189,
        200,
        210,
        211,
        290];
    for i in nearHundred_tests {
        println!("nearHundred: {}", nearHundred(i));
    }

    println!("\n");

    let posNeg_tests = vec![
        (1, -1, false),
        (-1, 1, false),
        (-4, -5, true),
        (-4, -5, false),
        (-4, 5, false),
        (-4, 5, true),
        (1, 1, false),
        (-1, -1, false),
        (1, -1, true),
        (-1, 1, true),
        (1, 1, true),
        (-1, -1, true),
        (5, -5, false),
        (-6, 6, false),
        (-5, -6, false),
        (-2, -1, false),
        (1, 2, false),
        (-5, 6, true),
        (-5, -5, true)];
    for i in posNeg_tests {
        println!("posNeg: {}", posNeg(i.0, i.1, i.2));
    }
    
    println!("\n");

    let frontBack_tests = vec![
        "code",
        "a",
        "ab",
        "abc",
        "",
        "Chocolate",
        "aavJ",
        "hello"];
    for i in frontBack_tests {
        println!("frontBack: {}", frontBack(i.to_string()));
    }
    println!("\n");

    let front3_tests = vec![
        "Java",
        "Chocolate",
        "abc",
        "abcXYZ",
        "ab",
        "ab",
        "a",
        ""];

    for i in front3_tests {
        println!("front3: {}", front3(i.to_string()));
    }

    println!("\n");

    let backAround_tests = vec![
        "cat",
        "Hello",
        "a",
        "abc",
        "read",
        "boo"];
    for i in backAround_tests {
        println!("backAround: {}", backAround(i.to_string()));
    }

    println!("\n");

    let or35_tests = vec![
        3,
        10,
        8,
        15,
        5,
        9,
        4,
        7,
        6,
        17,
        18,
        29,
        20,
        21,
        22,
        45,
        99,
        100,
        101,
        121,
        122,
        123];
    for i in or35_tests {
        println!("{}", or35(i));
    }
    
    println!("\n");
    
    let front22_tests = vec![
        "kitten",
        "Ha",
        "abc",
        "ab",
        "a",
        "",
        "Logic"];
    for i in front22_tests {
        println!("front22: {}", front22(i.to_string()));
    }


}




