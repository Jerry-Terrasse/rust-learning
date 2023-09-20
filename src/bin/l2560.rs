pub struct Solution {}

impl Solution {
    pub fn check(nums: &Vec<i32>, k: i32, cap: i32) -> bool {
        let mut f = vec![0; nums.len()];
        f[0] = (nums[0] <= cap) as i32;
        if nums.len() == 1 {
            return f[0] >= k;
        }
        f[1] = std::cmp::max(f[0], (nums[1] <= cap) as i32);
        for i in 2..nums.len() {
            f[i] = std::cmp::max(f[i-1], f[i-2] + (nums[i] <= cap) as i32);
        }
        f[nums.len()-1] >= k
    }
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = 0;
        let mut r = 1_000_000_000;
        while l+1 < r {
            let mid = (l + r + 1) / 2;
            if Self::check(&nums, k, mid) {
                r = mid;
            } else {
                l = mid;
            }
        }
        r
    }
}

fn main() {
    println!("{}", Solution::min_capability(vec![2,3,5,9], 2));
    println!("{}", Solution::min_capability(vec![2,7,9,3,1], 2));
}