#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn merge_88(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = (m + n - 1) as usize;

        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }

        while j >= 0 {
            nums1[k] = nums2[j as usize];
            k -= 1;
            j -= 1;
        }
    }

    pub fn majority_element_169(nums: Vec<i32>) -> i32 {
        // condition: assume that the majority element always exists in the array
        let mut cnt = 0;
        let mut candidate = 0;
        nums.iter().for_each(|n| {
            if cnt == 0 {
                candidate = *n;
            }
            if *n == candidate {
                cnt += 1;
            } else {
                cnt -= 1;
            }
        });
        return candidate;
    }

    pub fn contains_duplicate_217(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        for i in 1..nums.len() {
            if nums[i] == nums[i - 1] {
                return true;
            }
        }
        false
    }
}
