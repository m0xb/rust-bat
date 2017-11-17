fn last_index_of(string: &str, character: char) -> Option<u32> {;
    let str_len = string.len();
    for index in 0..str_len {
        if string.chars().rev().nth(index).unwrap() == character {
            return Some(index as u32)
        }
    }
    None
}

fn is_x_balanced(string: &str) -> bool {
    let last_x = last_index_of(string, 'x');
    let last_y = last_index_of(string, 'y'); 
    match (last_x, last_y) {
        (None, None) => true,
        (Some(x_index), Some(y_index)) if y_index > x_index => true,
        _ => false,
    }
}





fn main() {
    println!("{}", is_x_balanced("sdjfhdjsfyyyyhfasxyjx"));
}        
