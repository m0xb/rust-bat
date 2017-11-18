pub fn sumDouble(a: i32, b: i32) -> i32 {
    if a == b {
        return 4*a;
    } else {
        return a + b;
    }

}


fn main() {
    println!("{}", sumDouble(10, 12));
    println!("{}", sumDouble(5, 5));
}

mod tests {
    use super::*;

         #[test]
         fn test_sumDouble() {
             assert_eq!(20, sumDouble(5,5));
             assert_eq!(0, sumDouble(0,0));
             assert_eq!(-20, sumDouble(-5, -5));
             assert_eq!(1, sumDouble(5, -4));
             assert_eq!(12, sumDouble(3, 3));
         }
}

