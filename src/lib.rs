#[macro_export]
macro_rules! rasm {
    ( $( $keyword:tt $value:expr );* ) => {
        let mut variables: Vec<&str> = vec![];

        $(
            match stringify!($keyword) {
                "out" => {
                    println!("{}", $value);
                }

                "var" => {
                    variables.push($value);
                }

                "outv" => {
                    println!("{}", variables[$value.parse::<usize>().unwrap()]);
                }

                _ => {
                    println!("Unknown keyword!");
                }
            }
        )*
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        rasm! {
            out "hello!";
            var "this is a variable!";
            outv "0"
        }
    }
}
