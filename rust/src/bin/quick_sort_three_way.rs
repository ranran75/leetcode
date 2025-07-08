fn main() {
    let nums = vec![3, 2, 6, 1, 3, 4, 5, 2, 1];
    let res = Solution::sort_array(nums);
    println!("{:?}", res);
}

pub struct Solution {}

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        Self::quick_sort(&mut nums, 0, len - 1);
        nums
    }

    fn quick_sort(arr: &mut [i32], low: usize, high: usize) {
        if low >= high {
            return;
        }
        let (pivot_left, pivot_right) = Self::partition(arr, low, high);
        if pivot_left > 0 {
            Self::quick_sort(arr, low, pivot_left - 1);
        }
        Self::quick_sort(arr, pivot_right + 1, high);
    }

    fn partition(arr: &mut [i32], low: usize, high: usize) -> (usize, usize) {
        Self::get_median_of_three(arr, low, high);
        let mut pivot_left = low;
        let mut pivot_right = high;
        let pivot = arr[pivot_left];
        let mut i = low + 1;
        while i <= pivot_right {
            match arr[i].cmp(&pivot) {
                std::cmp::Ordering::Less => {
                    pivot_left += 1;
                    arr.swap(pivot_left, i);
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    arr.swap(pivot_right, i);
                    pivot_right -= 1;
                }
                std::cmp::Ordering::Equal => {
                    i += 1;
                }
            }
        }
        arr.swap(pivot_left, low);
        (pivot_left, pivot_right)
    }

    fn get_median_of_three(nums: &mut [i32], low: usize, high: usize) {
        let mid = (low + high) / 2;
        if nums[low] > nums[mid] {
            nums.swap(low, mid);
        }
        if nums[low] > nums[high] {
            nums.swap(low, high);
        }
        if nums[mid] > nums[high] {
            nums.swap(mid, high);
        }
        nums.swap(low, mid);
    }
}
