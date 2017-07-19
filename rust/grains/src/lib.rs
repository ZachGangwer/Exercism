pub fn square(s: u32) -> u64 {
    let base: u64 = 2;

    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    return base.pow(s - 1);
}

pub fn total() -> u64 {
    let mut out: u64 = 0;
    let base: u64 = 2;

    for x in 0..64 {
        out += base.pow(x);
    }

    return out;
}
