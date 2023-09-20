pub struct Solution {}

impl Solution {
    pub fn min_count(coins: Vec<i32>) -> i32 {
        let mut ans = 0;
        for coin in coins {
            ans += (coin + 1) / 2;
        }
        ans
    }
}

pub fn main() {
    println!("{}", Solution::min_count(vec![4, 2, 1]));
    println!("{}", Solution::min_count(vec![2, 3, 10]));
}