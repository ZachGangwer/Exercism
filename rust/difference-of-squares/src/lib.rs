pub fn square_of_sum(num: i32) -> i32 {
    let mut out: i32 = 0;

    for i in 0..num + 1 {
        out += i;
    }

    return out * out;
}

pub fn sum_of_squares(num: i32) -> i32 {
    let mut out: i32 = 0;

    for i in 0..num + 1 {
        out += i * i;
    }

    return out;
}

pub fn difference(num: i32) -> i32 {
    return square_of_sum(num) - sum_of_squares(num);
}
