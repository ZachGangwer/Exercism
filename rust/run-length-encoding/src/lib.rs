pub fn decode(arg: &str) -> String {
    return String::from(arg);
}

pub fn encode(arg: &str) -> String {
    let mut input = arg.chars();
    let mut out = String::new();
    let mut count: u32 = 0;
    let mut curr: char;
    let mut prev: char = '❤';

    loop {
        match input.next() {
            Some(curr) => {
                if curr == prev {
                    count += 1;
                } else {
                    if count == 0 {
                        out = out + format!("{}", curr).as_str();
                    } else {
                        out = out + format!("{num}{lttr}", num = count, lttr = curr).as_str();
                    }
                    
                    count = 0;
                    prev = curr;
                }
            },
            None => {
                break;
            }
        }
    }

    return out;
}
