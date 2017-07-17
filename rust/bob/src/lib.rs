pub fn reply(input: &'static str) -> String {
    let mut temp = String::from(input.to_string());

    temp.shrink_to_fit();
    temp.trim();

    if temp.is_empty() {
        return String::from("Fine. Be that way!")
    }

    let last = temp.pop().unwrap();
    let mut output = String::new();
    
    match last {
        '?' => output.push_str("Sure."),
        '!' => output.push_str("Whoa, chill out!"),
        _ => output.push_str("Whatever."),
    }

    return output;
}
