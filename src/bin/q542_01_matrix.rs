use std::collections::VecDeque;

fn main() {
    println!("Solution to Q542");
}

pub struct Solution;
// Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.
//
// The distance between two adjacent cells is 1.

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // we will make two matrices, 
        // 1) Will store if we have visited a node
        let rows = mat.len();
        let cols = mat[0].len();
        let mut visited = vec![vec![0; cols]; rows];
        // 2) We will store the distance this node is from a 0
        let mut distances = vec![vec![i32::MAX; cols]; rows]; 
        // Now, we will make a queue containing locations for all of the zeros.
        let mut nodes_to_visit = VecDeque::new();
        for i in 0..rows {
            for j in 0..cols {
                if mat[i][j] == 0 {
                    visited[i][j] = 1;
                    distances[i][j] = 0;
                    nodes_to_visit.push_back((i, j));
                }
            }
        }
        // For every node we visit, look at its neighbors and see if they need to be visited. If
        // so, add it to the nodes_to_visit queue and update it's distance value.
        while let Some((i,j)) = nodes_to_visit.pop_front() {
            // look up
            if i != 0 && visited[i - 1][j] == 0 {
                visited[i-1][j] = 1;
                nodes_to_visit.push_back((i-1, j));
                distances[i-1][j] = distances[i][j] + 1;
            }
            // down
            if i != rows - 1 && visited[i + 1][j] == 0 {
                visited[i+1][j] = 1;
                nodes_to_visit.push_back((i+1, j));
                distances[i+1][j] = distances[i][j] + 1;
            }
            // left
            if j != 0 && visited[i][j - 1] == 0 {
                visited[i][j-1] = 1;
                nodes_to_visit.push_back((i, j-1));
                distances[i][j-1] = distances[i][j] + 1;
            }
            // right
            if j != cols - 1 && visited[i][j + 1] == 0 {
                visited[i][j+1] = 1;
                nodes_to_visit.push_back((i, j+1));
                distances[i][j+1] = distances[i][j] + 1;
            }
        }
        // Return the computed distances
        distances
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let output = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        assert_eq!(output, Solution::update_matrix(mat));
    }
    #[test]
    fn ex2() {
        let mat = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let output = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]];
        assert_eq!(output, Solution::update_matrix(mat));
    }
}
// Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
// Output: [[0,0,0],[0,1,0],[0,0,0]]
//
// Example 2:
//
// Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
// Output: [[0,0,0],[0,1,0],[1,2,1]]
