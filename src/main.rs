use std::env;

mod warmup_1;
mod string_2;
mod monkeyTrouble;
mod sumDouble;
mod icyHot;
mod loneTeen;
mod delDel;

macro_rules! run_bat {
    ( $fn:path, $( $x:expr ),* ) => {{
        println!("{}", stringify!($fn));
        $(
            println!("    --> {:?}", $x);
        )*

        $fn(
        $(
            Arg{raw: &$x}.parse().unwrap_or_else(|e| panic!(e)),
        )*
        ).to_string()
    }};
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let ref section = args[1];
    let ref bat_name = args[2];

    let ret = match section.as_ref() {
        "warmup-1" => match bat_name.as_ref() {
            "sleepIn" => run_bat!(warmup_1::sleep_in, args[3], args[4]),
            "diff21" => run_bat!(warmup_1::diff21, args[3]),
            _ => panic!(format!("Unknown bat: {}", bat_name)),
        },
        "string-2" => match bat_name.as_ref() {
            "doubleChar" => run_bat!(string_2::double_char, args[3]),
            "xyBalance"  => run_bat!(string_2::xy_balance, args[3]),
            _ => panic!(format!("Unknown bat: {}", bat_name)),
        },
        "monkeyTrouble" => match bat_name.as_ref() {
            "monkey_trouble" => run_bat!(monkeyTrouble::monkey_trouble, args[3], args[4]),
            _ => panic!(format!("Unknown bat: {}", bat_name)),
        },
        "sumDouble" => match bat_name.as_ref() {
            "sumDouble" => run_bat!(sumDouble::sumDouble, args[3], args[4]),
            _ => panic!(format!("Unknown bat: {}", bat_name)),
        },
        "icyHot" => match bat_name.as_ref() {
            "icyHot" => run_bat!(icyHot::icyHot, args[3], args[4]),
            _ => panic!(format!("Unknown bat: {}", bat_name)),
        },
        "loneTeen" => match bat_name.as_ref() {
            "loneTeen" => run_bat!(loneTeen::loneTeen, args[3], args[4]),
            _ => panic!(format!("Unknown bat: {}", bat_name)),
        },
        "delDel" => match bat_name.as_ref() {
            "delDel" => run_bat!(delDel::delDel, args[3]),
            _ => panic!("Unknwon bat: {}", bat_name),
        },
        _ => panic!(format!("Unknown section: {}", section)),
    };

    println!("result: {:?}", ret);
}


struct Arg<'a> {
    raw: &'a String
}

impl<'a> Arg<'a> {
    fn parse<F: ParsedArg>(&self) -> Result<F, String> {
        return ParsedArg::from_str(self.raw.as_ref());
    }
}

trait ParsedArg: Sized {
    fn from_str(s: &str) -> Result<Self, String>;
}

impl ParsedArg for i32 {
    fn from_str(s: &str) -> Result<Self, String> {
        return match s.parse() {
            Ok(i) => Ok(i),
            Err(e) => Err(format!("Can't parse {:?} as i32: {}", s, e)),
        }
    }
}

impl ParsedArg for bool {
    fn from_str(s: &str) -> Result<Self, String> {
        return match s.parse() {
            Ok(b) => Ok(b),
            Err(e) => Err(format!("Can't parse {:?} as bool: {}", s, e)),
        }
    }
}

impl ParsedArg for String {
    fn from_str(s: &str) -> Result<Self, String> {
        return Ok(s.into());
    }
}

