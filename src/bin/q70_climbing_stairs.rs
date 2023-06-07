fn main() {
    println!("Solution to Q70");
}

// You are climbing a staircase. It takes n steps to reach the top.
// Each time you can either climb 1 or 2 steps.
// In how many distinct ways can you climb to the top?

pub struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 1 {
            panic!("n can not be less than 1");
        }
        if n == 1 {
            return 1
        }
        // else, we are on the 2nd step
        let mut one_step_permutes = 1;
        let mut two_step_permutes = 0; 
        
        for _ in 2..=n {
            let two_copy = two_step_permutes;
            two_step_permutes = one_step_permutes; 
            one_step_permutes = two_copy + one_step_permutes;
        }
        one_step_permutes + two_step_permutes 
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        assert_eq!(Solution::climb_stairs(2), 2);
    }
    #[test]
    fn ex2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }
    // 1 1 1 1
    // 1 1 2
    // 1 2 1
    // 2 1 1
    // 2 2
    #[test]
    fn ex3() {
        assert_eq!(Solution::climb_stairs(4), 5);
    }
    // 1 1 1 1 1
    // 1 1 1 2
    // 1 1 2 1
    // 1 2 1 1
    // 2 1 1 1
    // 2 2 1
    // 1 2 2
    // 2 1 2
    #[test]
    fn ex4() {
        assert_eq!(Solution::climb_stairs(5), 8);
    }
    // look to see how many times the val can be divided by 2
    // for 4 the anser is 2
    // Each of these 2s can either be a real 2 or a 1 1 pair.
    //
}
