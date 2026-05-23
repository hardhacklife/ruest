pub fn to_title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

pub fn to_type_name(s: &str) -> String {
    to_title_case(s.trim_end_matches("s"))
}
