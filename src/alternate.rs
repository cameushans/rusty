
pub fn alternate<'a>(n: usize, first_value: &'a str, second_value: &'a str) -> Vec<&'a str> {
    let mut v: Vec<&'a str> = Vec::new();
    let mut counter = 1;

    while counter <= n {
        v.push(first_value);
        v.push(second_value);
        counter += 1;
     
    }
    v.resize(n, first_value);
    return v;
}
