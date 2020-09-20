pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn add_one_works() {
        println!("{}", env::var("OUT_DIR").unwrap());

        assert_eq!(add_one(1), 2);
    }
}
