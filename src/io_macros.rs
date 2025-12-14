#[macro_export]
macro_rules! read_line {
    () => {{
        use std::io::stdin;

        let mut result = String::new();
        match stdin().read_line(&mut result).unwrap() {
            0 => Err(()),
            _ => Ok(
                result
                    .trim_start()
                    .trim_end()
                    .to_owned()
            )
        }
    }};

    ($a: expr) => {{
        use std::io::{stdin, stdout, Write};

        print!("{}", $a);
        stdout().flush().unwrap();
        let mut result = String::new();
        match stdin().read_line(&mut result).unwrap() {
            0 => Err(()),
            _ => Ok(
                result
                .trim_start()
                .trim_end()
                .to_owned()
            )
        }
    }};
}

#[macro_export]
macro_rules! read_number {
    () => {{
        let result: Result<i32, _> = read_line!()
            .unwrap()
            .parse();
        result
    }};

    ($a: expr) => {{
        let result: Result<i32, _> = read_line!($a)
            .unwrap()
            .parse();
        result
    }};
}

#[macro_export]
macro_rules! pause {
    () => {{
        read_line!("Press Enter to continue...").unwrap();
    }};
}
