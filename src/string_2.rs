pub fn double_char(s: String) -> String {
    let mut doubled = String::with_capacity(s.len() * 2);
    for c in s.chars() {
        doubled.push(c);
        doubled.push(c);
    }
    return doubled;
}

fn last_index_of(string: &str, character: char) -> Option<u32> {
    for index in 0..string.len() {
        if string.chars().rev().nth(index).unwrap() == character {
            return Some((string.len() - index) as u32)
        }
    }
    None
}

fn is_x_balanced(string: &str) -> bool {
    let last_x = last_index_of(string, 'x');
    let last_y = last_index_of(string, 'y');
    match (last_x, last_y) {
        (None, None) => true,
        (None, _) => true,
        (Some(x_index), Some(y_index)) if y_index > x_index => true,
        _ => false,
    }
}

pub fn xy_balance(string: String) -> bool {
    is_x_balanced(string.as_ref())
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_is_x_balanced() {
        assert_eq!(true, is_x_balanced(""));
        assert_eq!(true, is_x_balanced("y"));
        assert_eq!(true, is_x_balanced("abcd"));
        assert_eq!(false, is_x_balanced("x"));
        assert_eq!(true, is_x_balanced("xy"));
        assert_eq!(false, is_x_balanced("yx"));
        assert_eq!(true, is_x_balanced("xhello y"));
        assert_eq!(true, is_x_balanced("$@&*( #$&*#xy"));
        assert_eq!(true, is_x_balanced("xyxyxyxyxyxyxyxy"));

    }
}

