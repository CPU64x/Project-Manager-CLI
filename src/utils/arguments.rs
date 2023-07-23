use std::env;

pub struct Arguments;

impl Arguments {
    pub fn new() -> Vec<String> {
        env::args().collect()
    }

    // get_flag_value returns the value of a flag
    // if the flag is not present, it returns None
    pub fn get_value(args: &[String], flag: &str) -> Option<String> {
        let mut iter = args.iter().enumerate();

        while let Some((index, arg)) = iter.next() {
            if arg == flag {
                if let Some(value) = args.get(index + 1) {
                    return Some(value.clone());
                }
            }
        }
        None
    }

    // check if certain flag is present
    pub fn has_flag(args: &[String], flag: &str) -> bool {
        args.iter().any(|arg| arg == flag)
    }
}