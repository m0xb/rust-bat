fn printbat() {
    let mut groupSum_rows = Vec::new();
    let mut column_widths = (3, 0, 0, 0);
    for i in groupSum_tests {
        let invocation = format!("groupSum({}, {:?}, {})", i.0, i.1, i.2);
        let expectation = i.3.to_string();
        let actual = groupSum(i.0, &i.1, i.2).to_string();
        let passed = i.3 == groupSum(i.0, &i.1, i.2);

        column_widths.1 = max(invocation.chars().count(), column_widths.1);
        column_widths.2 = max(expectation.chars().count(), column_widths.2);
        column_widths.3 = max(actual.chars().count(), column_widths.3);
        groupSum_rows.push((passed, invocation, expectation, actual));
    }
    for i in groupSum_rows {
        fn render_cell(s: &str, cwidth: usize) {
            print!("{}", s);
            let width_delta = cwidth - s.chars().count();
            let padding = " ".repeat(width_delta);
            print!("{}", padding);
        }
        let maybe_check = if i.0 { "✔" } else { "" };
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
}