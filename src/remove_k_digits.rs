pub struct Solution {}

impl Solution {
    pub fn remove_k_digits(num: String, k: i32) -> String {
        let mut remove_count = k as usize;
        let mut stack = vec![];

        for digit in num.bytes() {
            loop {
                match stack.last() {
                    Some(&last_digit) if last_digit > digit && remove_count > 0 => {
                        stack.pop();
                        remove_count -= 1;
                    }
                    _ => break,
                }
            }
            stack.push(digit);
        }
        stack
            .drain(..stack.len() - remove_count)
            .skip_while(|&digit| digit == b'0')
            .map(|digit| digit as char)
            .collect()
    }
}
