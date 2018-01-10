macro_rules! double_print {
    () => {{println!("Hello")}};
    ($i:ident) => {println!("{}", stringify!($i))};
    ($e:expr) => {println!("Goodbye")};
    ($s:stmt) => {println!("The statement is: {}", stringify!($s))};
    ($a:expr, $b:expr) => {println!("a: {}, b: {}", $a, $b)};
}

macro_rules! n_print {
    () => {panic!("Macro takes one or more arguments")};
    ( $($x:expr),*) => {{
    $(
        println!("{}", $x);
    )*
    }}
}

macro_rules! print_bat {
    ($invocation:expr, $expected:expr) => {println!("{} -> {}", stringify!($invocation), $expected)};

}

macro_rules! invoke_any {
    (call $fn_name:ident with arguments $($arg:expr),+ ) => {{
    //$fn_name(14, 15)
    print!("{}(", stringify!($fn_name));
    let mut separator = "";
    $(
    print!("{}{}", separator, stringify!($arg));
    separator = ", ";
    )+
    print!(") -> {}", $fn_name($($arg,)+));

    }}

}
fn add(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in nums {
        sum += i;
    }
    return sum
}
fn main() {
//    double_print!();
//    double_print!("some");
//    double_print!(asdf);
//    println!("{}", stringify!(sd;jlkgnasdjobfjo;aedsfjkosadbn));
//    double_print!(let x = 3);
//    double_print!("hello", "there");
//    n_print!("Hello", 500);
//    //prints out passed, stringify!(function), expected, actual.
//    let add_tests = vec![
//        (1, 2, 3),
//        (3, 2, 5)];
//    for i in add_tests {
//        print_bat!(add(i.1, i.2), i.2);
//    }

    invoke_any!(call add with arguments vec![5, 5, 5, 5, 6, 6, 6, 6]);

}
