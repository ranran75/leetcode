use leetcode::remove_k_digits::Solution;

fn main() {
    let res1 = Solution::remove_k_digits("10232".to_string(), 3);
    let res2 = Solution::remove_k_digits("10232".to_string(), 2);
    let res3 = Solution::remove_k_digits("10232".to_string(), 1);
    println!("{},1", res1);
    println!("{},1", res2);
    println!("{},1", res3);
}
