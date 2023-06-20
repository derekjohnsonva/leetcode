fn main() {
    println!("Solution to Q");
}

pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        // My strategy will be to divide the itervals into three groups
        // 1) Before any overlap
        // 2) Overlaing vals
        // 3) After overlap
        let mut before_overlap = Vec::new();
        let mut after_overlap = Vec::new();
        let mut overlap_min = new_interval[0];
        let mut overlap_max = new_interval[1];
        for interval in intervals {
            // if the end of the interval in before the start of the new interval push to before
            // list
            if interval[1] < new_interval[0] {
                before_overlap.push(interval);
            // if the start of the interval is before the end of the new interval, push to after
            // list
            } else if interval[0] > new_interval[1] {
                after_overlap.push(interval);
            } else {
                // in this case, we only need to keep track of the new min and max
                overlap_min = i32::min(overlap_min, interval[0]);
                overlap_max = i32::max(overlap_max, interval[1]);
            }
        }
        // combine the three vectors
        before_overlap.push(vec![overlap_min, overlap_max]);
        before_overlap.append(&mut after_overlap);
        before_overlap
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let output = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(output, Solution::insert(intervals, new_interval));
    }
    #[test]
    fn ex2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        let output = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
        assert_eq!(output, Solution::insert(intervals, new_interval));
    }
}

// Example 1:
//
// Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
// Output: [[1,5],[6,9]]
//
// Example 2:
//
// Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
// Output: [[1,2],[3,10],[12,16]]
// Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
//
