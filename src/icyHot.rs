pub fn icyHot(temp1: i32, temp2: i32) -> bool {
    if (temp1 < 0 && temp2 > 100) || (temp1 > 0 && temp2 < 0) {
        return true;
    } else {
        return false;
    }
}

fn main() {
    println!("{}", icyHot(32, 120));
    println!("{}", icyHot(-1, 120));
    println!("{}", icyHot(0, 0));
    println!("{}", icyHot(120, -1));
} 
        
