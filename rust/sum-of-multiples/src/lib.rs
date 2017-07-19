pub fn sum_of_multiples(bound: u64, mults: &Vec<u64>) -> u64 {
    let mut temp = Vec::new();
    let mut i = mults.iter();
    let mut j: u64;

    loop {
        match i.next() {
            Some(x) => {
                j = *x;
                while j < bound {
                    temp.push(j);
                    j += *x;
                }
            }
            None => break,
        }
    }

    temp.sort();
    temp.dedup();
    return temp.iter().sum();
}
