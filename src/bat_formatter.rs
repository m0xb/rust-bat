macro_rules! printbat {
($bat:ident, $($($arg:expr),+ => $expectation:expr),+) => {{
    let mut rows = Vec::new();
    let mut column_widths = (3, 0, 0, 0);
    use std::cmp;
    $(
        let mut invocation = stringify!($bat).to_owned();
        invocation += "(";
        let mut separator = "";
        $(
            invocation += &format!("{}{:?}", separator, $arg);
            separator = ", ";
        )+
        invocation += ")";
        //let invocation = format!("{}({:?})", stringify!($bat), $($arg,)+);
        let expectation = format!("{:?}", $expectation);
        let actual = $bat($($arg,)+);
        let actual_str = format!("{:?}", actual);
        let passed: bool = actual == $expectation;
        column_widths.1 = cmp::max(invocation.chars().count(), column_widths.1);
        column_widths.2 = cmp::max(expectation.chars().count(), column_widths.2);
        column_widths.3 = cmp::max(actual_str.chars().count(), column_widths.3);
        rows.push((passed, invocation, expectation, actual_str));
    )+
    let mut correct = 0;
    let mut total = 0;
    for i in rows {
        total += 1;
        fn render_cell(s: &str, cwidth: usize) {
            print!("{}", s);
            let width_delta = cwidth - s.chars().count();
            let padding = " ".repeat(width_delta);
            print!("{}", padding);
        }
        let maybe_check = if i.0 { "✔" } else { "X" };
        if maybe_check == "✔" {
            correct += 1
        }
        if !i.0 {
            print!("\x1B[31m")
        }
        render_cell(maybe_check, column_widths.0);
        render_cell(&i.1, column_widths.1);
        print!(" ");
        render_cell(&i.2, column_widths.2);
        print!(" ");
        render_cell(&i.3, column_widths.3);
        if !i.0 {
            print!("\x1B[0m")
        }
        println!();
    }

    if correct == total {
    println!("\x1B[32m OK: {} / {} \n", correct, total,);
    print!("\x1B[0mm");
}
    else {
    println!("\x1B[31m FAILURES: {} / {}, you missed: {} \n", correct, total, total - correct);
    print!("\x1B[0mm");
    }
}
}
}