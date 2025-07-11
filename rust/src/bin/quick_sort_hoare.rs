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
        let pivot_index = Self::partition(arr, low, high);
        if pivot_index > 0 {
            Self::quick_sort(arr, low, pivot_index);
        }
        Self::quick_sort(arr, pivot_index + 1, high);
    }

    fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
        Self::get_median_of_three(arr, low, high);
        let mut left = low;
        let mut right = high;
        let pivot = arr[left];
        while left < right {
            while arr[right] >= pivot && right > left {
                right -= 1;
            }
            while arr[left] <= pivot && right > left {
                left += 1;
            }
            arr.swap(left, right);
        }
        arr.swap(low, right);
        right
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
