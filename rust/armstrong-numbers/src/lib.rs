pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum = 0;
    let num_str = num.to_string();
    let num_len = num_str.len();
    for c in num_str.chars() {
        let digit = c.to_digit(10).unwrap();
        sum += digit.pow(num_len as u32);
    }
    sum == num
}
