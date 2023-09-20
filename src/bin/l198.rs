pub struct Solution {}

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        } else if nums.len() == 2 {
            return nums[0].max(nums[1]);
        }

        // let mut nums = nums;
        // nums[2] += nums[0];
        // for i in 3..nums.len() {
        //     nums[i] += nums[i-3].max(nums[i-2]);
        // }
        // return nums[nums.len()-1].max(nums[nums.len()-2]);
        let mut ans = [0; 3];
        ans[0] = nums[0]; ans[1] = nums[1]; ans[2] = nums[0] + nums[2];
        for i in 3..nums.len() {
            ans[i%3] = nums[i] + ans[(i-2)%3].max(ans[(i-3)%3]);
        }
        return ans[(nums.len()-1)%3].max(ans[(nums.len()-2)%3]);
    }
}

fn main() {
    println!("{}", Solution::rob(vec![1,2,3,1]));
    println!("{}", Solution::rob(vec![2,7,9,3,1]));
}