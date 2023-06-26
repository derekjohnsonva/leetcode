fn main() {
    println!("Solution to Q973");
}

pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        assert!(!points.is_empty());
        // make a sorted list of points    
        let mut k_closest: Vec<(i32, Vec<i32>)> = Vec::with_capacity(k as usize);
        for point in points.into_iter() {
            let point_dist = i32::pow(point[0], 2) + i32::pow(point[1], 2);

            match k_closest.get(k as usize - 1) {
                Some((max_dist, _)) => {
                    if point_dist < *max_dist {
                        // update the last entry
                        k_closest[k as usize - 1] = (point_dist, point);
                        // resort the list so that the last entry is the largest
                        k_closest.sort_by(|a, b| {a.0.cmp(&b.0)});
                        // print k_closest 
                        println!("{:?}", k_closest);
                    }
                },
                None => {
                    k_closest.push((point_dist, point));
                    k_closest.sort_by(|a, b| {a.0.cmp(&b.0)});
                }
            } 
        }
        // turn k_closest into the output type
        k_closest.iter().map(|(_, point)| point.to_vec()).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let points = vec![vec![1,3],vec![-2,2]];
        let k = 1;
        let output = vec![vec![-2, 2]]; 
        assert_eq!(output, Solution::k_closest(points, k));
    }
    #[test]
    fn ex2() {
        let points = vec![vec![1,3],vec![-2,2],vec![2,-2]];
        let k = 2;
        let output = vec![vec![2,-2],vec![-2,2]]; 
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
