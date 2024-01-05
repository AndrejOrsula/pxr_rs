use std::borrow::Cow;

pub fn remove_redundant_whitespace(input: &str) -> Cow<str> {
    let re = regex::Regex::new(r"\s+").unwrap();
    re.replace_all(input, " ")
}
