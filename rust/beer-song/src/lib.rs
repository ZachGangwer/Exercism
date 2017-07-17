pub fn sing(start: i32, end: i32) -> String {
    let mut out = String::new();
    let mut count = start;

    while count >= end {
        out.push_str(verse(count).as_str());
        out.push('\n');
        count -= 1;
    }
    
    out.pop();

    return out;
}

pub fn verse(num: i32) -> String {
    let mut out = String::new();

    match num {
        0 => {
            out.push_str(format!("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n").as_str());
        },
        1 => {
            out.push_str(format!("{x} bottle of beer on the wall, {x} bottle of beer.\n", x = num).as_str());
            out.push_str(format!("Take it down and pass it around, no more bottles of beer on the wall.\n").as_str());
        },
        2 => {
            out.push_str(format!("{x} bottles of beer on the wall, {x} bottles of beer.\n", x = num).as_str());
            out.push_str(format!("Take one down and pass it around, {x} bottle of beer on the wall.\n", x = num - 1).as_str());
        },
        _ => {
            out.push_str(format!("{x} bottles of beer on the wall, {x} bottles of beer.\n", x = num).as_str());
            out.push_str(format!("Take one down and pass it around, {x} bottles of beer on the wall.\n", x = num - 1).as_str());
        }
    }

    return out;
}
