use regex::Regex;

pub fn extract_otp(input: &str) -> Option<String> {
    let re: Regex = Regex::new(r"(?i)otp(?:\s*code)?\D*(\d{6})").unwrap();

    re.captures(input).and_then(|caps| caps.get(1)).map(|m| m.as_str().to_string())
}
