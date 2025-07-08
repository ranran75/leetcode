/**
<https://leetcode.cn/problems/find-nearest-point-that-has-the-same-x-or-y-coordinate/>
*/
fn main() {
    let res1 = Solution::nearest_valid_point(
        3,
        4,
        vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]],
    );
    println!("{},res1", res1);
    let res2 = Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]);
    println!("{},res2", res2);
}

struct Solution {}

impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let (mut nearest, mut nearest_idx) = (i32::MAX, -1);
        points.iter().enumerate().for_each(|(idx, point)| {
            if point[0] == x {
                (nearest, nearest_idx) =
                    Self::is_nearest(nearest, nearest_idx, idx as i32, point[1], y)
            } else if point[1] == y {
                (nearest, nearest_idx) =
                    Self::is_nearest(nearest, nearest_idx, idx as i32, point[0], x)
            }
        });
        nearest_idx
    }
    fn is_nearest(nearest: i32, pre_idx: i32, idx: i32, i: i32, j: i32) -> (i32, i32) {
        let dist = (i - j).abs();
        if dist >= nearest {
            (nearest, pre_idx)
        } else {
            (dist, idx)
        }
    }
}
