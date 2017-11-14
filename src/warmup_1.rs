pub fn sleep_in(weekday: bool, vacation: bool) -> bool {
    return !weekday || vacation;
}

pub fn diff21(n: i32) -> i32 {
    return (21 - n).abs() * (1 + ((n > 21) as i32))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diff21() {
        assert_eq!(2, diff21(19));
        assert_eq!(11, diff21(10));
        assert_eq!(0, diff21(21));
        assert_eq!(2, diff21(22));
        assert_eq!(8, diff21(25));
        assert_eq!(18, diff21(30));
        assert_eq!(21, diff21(0));
        assert_eq!(20, diff21(1));
        assert_eq!(19, diff21(2));
        assert_eq!(22, diff21(-1));
        assert_eq!(23, diff21(-2));
        assert_eq!(58, diff21(50));
        assert_eq!(100, diff21(71));
    }
}

