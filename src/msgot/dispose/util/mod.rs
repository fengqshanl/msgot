use regex::Regex;
pub fn is_subdomain(url: &str) -> bool {
    let re = Regex::new(r"(https?://)").unwrap();
    return if re.is_match(&url) {
        false
    } else {
        true
    }
}