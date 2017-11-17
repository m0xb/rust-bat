pub fn loneTeen(int1: i32, int2: i32) -> bool {
    if (13 <= int1 && int1 <= 19) ^ (13 <= int2 && int2 <= 19) {
        return true;
    } else {
        return false;
    }
}

fn main() {
    println!("{}", loneTeen(14, 21));
    println!("{}", loneTeen(21, 14));
    println!("{}", loneTeen(14, 14));
    println!("{}", loneTeen(21, 21));
}

