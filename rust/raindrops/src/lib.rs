pub fn raindrops(int: u64) -> String {
    let mut out = String::new();

    if int % 3 == 0 {
        out.push_str("Pling");
    }
    if int % 5 == 0 {
        out.push_str("Plang");
    }
    if int % 7 == 0 {
        out.push_str("Plong");
    }
    if out.is_empty() {
        out.push_str(&format!("{}", int));
    }

    out.shrink_to_fit();
    
    return out;
}
