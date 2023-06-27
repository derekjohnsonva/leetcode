fn main() {
    println!("Solution to Q973");
}

pub struct Solution;

impl Solution {
    pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;
        points.select_nth_unstable_by_key(k - 1, |p| p[0].pow(2) + p[1].pow(2));
        points.truncate(k);
        points
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let output = vec![vec![-2, 2]];
        assert_eq!(output, Solution::k_closest(points, k));
    }
}

// Input: points = [[1,3],[-2,2]], k = 1
// Output: [[-2,2]]
// Explanation:
// The distance between (1, 3) and the origin is sqrt(10).
// The distance between (-2, 2) and the origin is sqrt(8).
// Since sqrt(8) < sqrt(10), (-2, 2) is closer to the origin.
// We only want the closest k = 1 points from the origin, so the answer is just [[-2,2]].
//
// Example 2:
//
// Input: points = [[3,3],[5,-1],[-2,4]], k = 2
// Output: [[3,3],[-2,4]]
// Explanation: The answer [[-2,4],[3,3]] would also be accepted.
