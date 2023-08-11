fn main() {
    println!("Solution to Q322");
}

pub struct Solution;

impl Solution {
    fn min_coin(target: i32, coins: Vec<i32>, dp: &mut Vec<Vec<i32>>) -> i32 {
        // We want to find the minimum number of coins needed to make the target.
        // Assume that coins is sorted in increasing order.
        // Base case: coins is an empty list OR target is 0
        if target == 0 {
            return 0;
        }
        let mut answer = i32::MAX; 
        if let Some((last, slice)) = coins.split_last() {
            // check to see if we have already solved this subproblem
            if dp[coins.len() - 1][target as usize] != -1 {
                return dp[coins.len() - 1][target as usize]; 
            }
            // If the largest coin is greater than the target, retry with a smaller set of coins
            if *last > target {
                answer = Solution::min_coin(target, slice.to_vec(), dp);
            } else {
                // If this is not the case, try adding the biggest value and not and see which value is
                // smaller
                let add_biggest = match Solution::min_coin(target - *last, coins.to_vec(), dp) {
                    i32::MAX => i32::MAX,
                    x => x + 1,
                };
                let shrink_slice = Solution::min_coin(target, slice.to_vec(), dp);            
                answer = i32::min(add_biggest, shrink_slice);
            }     
            dp[coins.len() - 1][target as usize] = answer;
            return answer;
        }
        answer
    }
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        // basically, this is going to be a bunch of modulo operations.
        // First, sort the coins in increasing order
        coins.sort_unstable();
        let mut dp = vec![vec![-1 ; (amount + 1) as usize] ; coins.len()];
        let sol = Solution::min_coin(amount, coins, &mut dp);     
        if sol == i32::MAX {
            return -1;
        }
        sol
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let coins = vec![1,2,5];
        let amount = 11;
        assert_eq!(Solution::coin_change(coins, amount), 3);
    }
    #[test]
    fn ex2() {
        let coins = vec![2];
        let amount = 3;
        assert_eq!(Solution::coin_change(coins, amount), -1);
    }
    #[test]
    fn ex3() {
        let coins = vec![1];
        let amount = 0;
        assert_eq!(Solution::coin_change(coins, amount), 0);
    }
}

// Example 1:
//
// Input: coins = [1,2,5], amount = 11
// Output: 3
// Explanation: 11 = 5 + 5 + 1
//
// Example 2:
//
// Input: coins = [2], amount = 3
// Output: -1
//
// Example 3:
//
// Input: coins = [1], amount = 0
// Output: 0
