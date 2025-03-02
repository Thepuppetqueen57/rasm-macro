#[macro_export]
macro_rules! rasm {
    ( $( $keyword:ident $value:expr );* ) => {
        let mut variables: Vec<&str> = vec![];

        $(
            if stringify!($keyword) == "out" {
                println!($value);
            }

            else if stringify!($keyword) == "var" {
                variables.push($value);
            }

            else if stringify!($keyword) == "outv" {
                println!("{}", variables[$value.parse::<usize>().unwrap()]);
            }

            else {
                println!("Unknown keyword!");
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
