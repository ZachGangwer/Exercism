pub fn decode(arg: &str) -> String {
    let mut input: Vec<u8> = arg.as_bytes().to_owned();
    let mut out = String::new();
    let mut temp: u8;
    let mut count: u32;

    while input.len() != 0 {
        count = 0;
        temp = input.pop().unwrap();

        while temp == *input.last().unwrap() {
            count += 1;
            input.pop();
        }

        out = String::from(format!(
            "{num}{chr}{prev}",
            num = count,
            chr = temp,
            prev = out.as_str()
        ));
    }

    return out;
}

pub fn encode(arg: &str) -> String {
    let mut temp = String::from(arg);

    return String::from("");
}
