
fn main() {
    println!("Solution to Q207");
}

pub struct Solution;

impl Solution {
   fn is_cyclic(i: usize, graph: &Vec<Vec<usize>>, visited: &mut Vec<bool>, stack: &mut Vec<bool>) -> bool {
        if !visited[i] {
            visited[i] = true;
            stack[i] = true;

            for &j in &graph[i] {
                if !visited[j] && Self::is_cyclic(j as usize, graph, visited, stack) {
                    return true;
                } else if stack[j] {
                    return true;
                }
            }
        }
        stack[i] = false;
        return false;
    } 
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        // My thought is that this can be completed by building a graph and looking for cycles.
        // A cycle would represent a circular dependency.
        // First, build the graph. We can do this with vec of vecs. The inner vec represents the 
        // edges from the node 
        let mut graph: Vec<Vec<usize>> = vec![vec![]; num_courses as usize];
        for edge in prerequisites.iter() {
            let src = edge[1] as usize;
            let dst = edge[0] as usize;
            graph[src].push(dst);
        }

        let mut stack = vec![false; num_courses as usize];
        let mut visited = vec![false; num_courses as usize];
        for i in 0..num_courses as usize {
            if !visited[i] && Self::is_cyclic(i, &graph, &mut visited, &mut stack) {
                return false;
            }
        }
        true
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let num_courses = 2;
        let prerequisites = vec![vec![1,0]];
        assert!(Solution::can_finish(num_courses, prerequisites));
    }
    #[test]
    fn ex2() {
        let num_courses = 2;
        let prerequisites = vec![vec![1,0],vec![0,1]];
        assert!(!Solution::can_finish(num_courses, prerequisites));
    }
    #[test]
    fn ex3() {
        let num_courses = 3;
        let prerequisites = vec![vec![0,1],vec![0,2], vec![1,2]];
        assert!(Solution::can_finish(num_courses, prerequisites));
    }
}


// Example 1:
//
// Input: numCourses = 2, prerequisites = [[1,0]]
// Output: true
// Explanation: There are a total of 2 courses to take. 
// To take course 1 you should have finished course 0. So it is possible.
//
// Example 2:
//
// Input: numCourses = 2, prerequisites = [[1,0],[0,1]]
// Output: false
// Explanation: There are a total of 2 courses to take. 
// To take course 1 you should have finished course 0, and to take course 0 you should also have finished course 1. So it is impossible.
