#[macro_use]
mod bat_formatter;
fn main() {
    fn test_bat(s: &str) -> String {
        return "hi".to_owned();
    }
    let test = ("hello", "hi");
    let test2 = ("My name isssssssss: ", "Aaaaaaaaaaaaaaaaaaaaaalan");
    printbat!(test_bat, test.0 => test.1,
    test2.0 => test2.1,
    "hi" => "hi");
    fn test_batN(s: &str, n: i32, v: Vec<i32>) -> String {
        return s.to_owned();
    }
    printbat!(test_batN, "I have a number and a vector", 12345324, vec![12, 3, 2, -4] => "I have a number and a vector")

}