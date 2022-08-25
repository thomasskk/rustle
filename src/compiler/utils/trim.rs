use super::{END_WHITESPACE, START_WHITESPACE};

pub fn trim_start(str: &str) -> String {
    START_WHITESPACE.replace_all(str, "").to_string()
}

pub fn trim_end(str: &str) -> String {
    END_WHITESPACE.replace_all(str, "").to_string()
}
