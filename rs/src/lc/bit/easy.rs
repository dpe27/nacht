#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn add_binary_67_1(a: String, b: String) -> String {
        let a_bytes: &[u8] = a.as_bytes();
        let b_bytes: &[u8] = b.as_bytes();

        let mut res: Vec<char> = vec![];
        let mut m: usize = a.len();
        let mut n: usize = b.len();
        let mut flag = false;

        while m > 0 || n > 0 {
            let mut c: char;

            if m > 0 && n > 0 {
                m -= 1;
                n -= 1;

                if a_bytes[m] == b'1' && b_bytes[n] == b'1' {
                    if !flag {
                        c = '0';
                        flag = true;
                    } else {
                        c = '1';
                    }
                    res.push(c);
                    continue;
                }

                if a_bytes[m] == b'1' || b_bytes[n] == b'1' {
                    c = '1';
                } else {
                    c = '0';
                }

                if flag {
                    if c == '1' {
                        c = '0';
                    } else {
                        c = '1';
                        flag = false;
                    }
                }

                res.push(c);
                continue;
            }

            if m > 0 {
                m -= 1;
                if (a_bytes[m] == b'0' && !flag) || (a_bytes[m] == b'1' && flag) {
                    res.push('0');
                }

                if (a_bytes[m] == b'0' && flag) || (a_bytes[m] == b'1' && !flag) {
                    res.push('1');
                    flag = false;
                }

                continue;
            }

            if n > 0 {
                n -= 1;
                if (b_bytes[n] == b'0' && !flag) || (b_bytes[n] == b'1' && flag) {
                    res.push('0');
                }

                if (b_bytes[n] == b'0' && flag) || (b_bytes[n] == b'1' && !flag) {
                    res.push('1');
                    flag = false;
                }

                continue;
            }
        }

        if flag {
            res.push('1');
        }

        res.reverse();
        res.into_iter().collect()
    }

    pub fn add_binary_67_2(a: String, b: String) -> String {
        let max_len = a.len().max(b.len()) + 1;
        let mut carry = 0;

        let mut res: String = a
            .chars()
            .rev()
            .chain(std::iter::repeat('0'))
            .zip(b.chars().rev().chain(std::iter::repeat('0')))
            .take(max_len)
            .map(|(l, r)| {
                let mut count = 0;

                if l == '1' {
                    count += 1;
                }
                if r == '1' {
                    count += 1;
                }

                count += carry;
                carry = count / 2;

                if count % 2 == 1 {
                    '1'
                } else {
                    '0'
                }
            })
            .collect();

        if res.len() > 1 && res.ends_with('0') {
            res.remove(res.len() - 1);
        }

        res.chars().rev().collect()
    }

    pub fn single_number_136(nums: Vec<i32>) -> i32 {
        nums.iter().fold(0, |acc, num| *num ^ acc)
    }

    pub fn reverse_bits_190_1(mut x: u32) -> u32 {
        if x == 0 {
            return 0;
        }
        (0..32).fold(0, |mut res, _| {
            res <<= 1;
            res |= x & 1;
            x >>= 1;
            return res;
        })
    }

    pub fn reverse_bits_190_2(mut x: u32) -> u32 {
        x = (x >> 16) | (x << 16);
        //x = ((x & 0b11111111111111110000000000000000) >> 16) | ((x & 0b00000000000000001111111111111111) << 16);
        x = ((x & 0xff00ff00) >> 8) | ((x & 0x00ff00ff) << 8);
        //x = ((x & 0b11111111000000001111111100000000) >> 8) | ((x & 0b00000000111111110000000011111111) << 8);
        x = ((x & 0xf0f0f0f0) >> 4) | ((x & 0x0f0f0f0f) << 4);
        // x = ((x & 0b11110000111100001111000011110000) >> 4) | ((x & 0b00001111000011110000111100001111) << 4);
        x = ((x & 0xcccccccc) >> 2) | ((x & 0x33333333) << 2);
        // x = ((x & 0b11001100110011001100110011001100) >> 2) | ((x & 0b00110011001100110011001100110011) << 2);
        x = ((x & 0xaaaaaaaa) >> 1) | ((x & 0x55555555) << 1);
        // x = ((x & 0b10101010101010101010101010101010) >> 1) | ((x & 0b01010101010101010101010101010101) << 1);
        return x;
    }

    pub fn hamming_weight_191_1(mut n: i32) -> i32 {
        let mut res = 0;
        while n != 0 {
            res += n & 1;
            n >>= 1;
        }
        res
    }

    pub fn hamming_weight_191_2(mut n: i32) -> i32 {
        // method 2: stripping off the lowest set bit
        let mut res = 0;
        while n != 0 {
            n &= n - 1;
            res += 1;
        }
        res
    }

    pub fn hamming_weight_191_3(mut n: i32) -> i32 {
        // method 3: divide and conquer
        n = n - ((n >> 1) & 0x55555555);
        n = (n & 0x33333333) + ((n >> 2) & 0x33333333);
        n = (n + (n >> 4)) & 0x0f0f0f0f;
        n += n >> 8;
        n += n >> 16;
        return n & 0x3f;
    }

    pub fn is_power_of_two_231(n: i32) -> bool {
        n > 0 && (n & (n - 1) == 0)
    }

    pub fn missing_number_268_1(nums: Vec<i32>) -> i32 {
        (0..nums.len()).fold(nums.len() as i32, |res, i| res ^ (i as i32) ^ nums[i])
    }

    pub fn missing_number_268_2(nums: Vec<i32>) -> i32 {
        (0..nums.len()).fold(nums.len() as i32, |res, i| res - nums[i] + i as i32)
    }

    pub fn count_bits_338(n: i32) -> Vec<i32> {
        let mut dp = vec![0i32; n as usize + 1];
        for i in 1..=n as usize {
            dp[i] = dp[i >> 1] + (i & 1) as i32;
        }
        dp
    }

    pub fn is_power_of_four_342(n: i32) -> bool {
        n > 0 && (n & (n - 1) == 0) && n & 0x55555555 != 0
    }

    pub fn find_the_difference_389(s: String, t: String) -> char {
        s.as_bytes()
            .iter()
            .chain(t.as_bytes().iter())
            .fold(0u8, |diff, b| *b ^ diff) as char
    }

    pub fn read_binary_watch_401(turned_on: i32) -> Vec<String> {
        let mut res = vec![];
        for h in 0..12 as u32 {
            for m in 0..60 as u32 {
                if (h << 6 | m).count_ones() == turned_on as u32 {
                    res.push(format!(
                        "{}:{}",
                        h,
                        if m < 10 {
                            format!("0{}", m)
                        } else {
                            m.to_string()
                        }
                    ))
                }
            }
        }
        res
    }

    pub fn to_hex_405(mut num: i32) -> String {
        if num == 0 {
            return String::from("0");
        }

        const HEX_CHARS: [char; 16] = [
            '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
        ];

        let mut res = String::new();
        while num != 0 && res.len() < 8 {
            res = HEX_CHARS[(num & 0xf) as usize].to_string() + &res;
            num >>= 4;
        }
        res
    }

    pub fn hamming_distance_461(x: i32, y: i32) -> i32 {
        let mut k = x ^ y;
        let mut res = 0;
        while k != 0 {
            k &= k - 1;
            res += 1;
        }
        res
    }

    pub fn find_complement_476_1(num: i32) -> i32 {
        // use log2
        let k = num.ilog2();
        let mask = (1 << k) | ((1 << k) - 1);
        !num & mask
    }

    pub fn find_complement_476_2(num: i32) -> i32 {
        if num == 0 {
            return 1;
        }

        let mut k = num;
        k |= k >> 1;
        k |= k >> 2;
        k |= k >> 4;
        k |= k >> 8;
        k |= k >> 16;

        return !num & k;
    }

    pub fn find_error_nums_645(nums: Vec<i32>) -> Vec<i32> {
        let mut used = vec![false; nums.len() + 1];
        let mut k = 0;
        let mut a = nums[0];

        for i in 0..nums.len() {
            k ^= (i as i32 + 1) ^ nums[i];
            if used[nums[i] as usize] {
                a = nums[i];
            } else {
                used[nums[i] as usize] = true;
            }
        }

        vec![a, k ^ a]
    }

    pub fn has_alternating_bits_693_1(n: i32) -> bool {
        let n = n as u32;
        let mut k = n;
        k |= k >> 1;
        k |= k >> 2;
        k |= k >> 4;
        k |= k >> 8;
        k |= k >> 16;

        let mask = 0xaaaaaaaa & k;
        (n ^ mask == 0) || (n ^ mask == k)
    }

    pub fn has_alternating_bits_693_2(mut n: i32) -> bool {
        n = n ^ (n >> 1);
        n & (n + 1) == 0
    }

    pub fn count_prime_set_bits_762(left: i32, right: i32) -> i32 {
        use std::collections::HashSet;
        let primes: HashSet<u8> = HashSet::from([2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]);

        let hamming_weight = |mut n: i32| -> u8 {
            let mut cnt = 0;
            while n != 0 {
                n &= n - 1;
                cnt += 1;
            }
            cnt
        };

        (left..=right).fold(0i32, |res, n| {
            if primes.contains(&hamming_weight(n)) {
                return res + 1;
            }
            res
        })
    }

    pub fn flip_and_invert_image_832(mut image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = image.len();
        image.iter_mut().for_each(|row| {
            for i in 0..n / 2 {
                if row[i] == row[n - 1 - i] {
                    row[n - i - 1] ^= 1;
                    row[i] = row[n - i - 1];
                }
            }

            if n % 2 != 0 {
                row[n / 2] ^= 1;
            }
        });
        image
    }

    pub fn binary_gap_868(mut n: i32) -> i32 {
        let mut max_gap = 0;
        let mut prev = n & -n;
        while n != 0 {
            let curr = n & -n;
            max_gap = max_gap.max(curr.trailing_zeros() - prev.trailing_zeros());
            prev = curr;
            n &= n - 1;
        }
        max_gap as i32
    }

    pub fn bitwise_complement_1009(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let k = n.ilog2();
        let mask = (1 << k) | ((1 << k) - 1);
        !n & mask
    }

    pub fn prefixes_div_by5_1018(nums: Vec<i32>) -> Vec<bool> {
        let mut prev = nums[0] % 5;
        (1..nums.len()).fold(vec![nums[0] % 5 == 0], |mut res, i| {
            prev = ((prev << 1) | nums[i]) % 5;
            res.push(prev == 0);
            return res;
        })
    }

    pub fn number_of_steps_1342(mut num: i32) -> i32 {
        let mut res = 0;
        while num != 0 {
            if num & 1 == 1 {
                num &= !1;
            } else {
                num >>= 1;
            }
            res += 1;
        }
        res
    }

    pub fn sort_by_bits_1356(mut arr: Vec<i32>) -> Vec<i32> {
        arr.sort_by(|a, b| a.count_ones().cmp(&b.count_ones()).then_with(|| a.cmp(b)));
        return arr;
    }

    pub fn xor_operation_1486(n: i32, start: i32) -> i32 {
        (0..n).fold(0, |res, i| res ^ (start + 2 * i))
    }

    pub fn decode_1720(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut res = vec![first];
        res.extend(encoded);
        (1..res.len()).for_each(|i| res[i] ^= res[i - 1]);
        res
    }

    pub fn longest_nice_substring_1763(s: String) -> String {
        let mut start = 0;
        let mut end = 1;
        let bytes = s.into_bytes();

        for i in 0..bytes.len() - 1 {
            let mut lower_case = 0;
            let mut upper_case = 0;

            for j in i..bytes.len() {
                if bytes[j] >= b'a' {
                    lower_case |= 1 << (bytes[j] - b'a');
                } else {
                    upper_case |= 1 << (bytes[j] - b'A');
                }
                if upper_case == lower_case && j + 1 - i > end - start {
                    start = i;
                    end = j + 1;
                }
            }
        }

        unsafe { String::from_utf8_unchecked(bytes[start..end].to_vec()) }
    }

    pub fn subset_xor_sum_1863(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        fn solve(idx: usize, acc: i32, res: &mut i32, nums: &Vec<i32>) {
            *res += acc;

            if idx == nums.len() {
                return;
            }

            for i in idx..nums.len() {
                solve(i + 1, acc ^ nums[i], res, nums);
            }
        }

        solve(0, 0, &mut res, &nums);
        res
    }

    pub fn two_out_of_three_2032(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
        let mut freq = [0u8; 101];
        nums1.iter().for_each(|n| freq[*n as usize] |= 1 << 0);
        nums2.iter().for_each(|n| freq[*n as usize] |= 1 << 1);
        nums3.iter().for_each(|n| freq[*n as usize] |= 1 << 2);
        (1..freq.len()).fold(vec![], |mut acc, i| {
            if freq[i] == 3 || freq[i] >= 5 {
                acc.push(i as i32);
            }
            return acc;
        })
    }

    pub fn divide_array_2206(mut nums: Vec<i32>) -> bool {
        nums.sort_unstable();
        for i in (0..nums.len()).step_by(2) {
            if nums[i] ^ nums[i + 1] != 0 {
                return false;
            }
        }
        return true;
    }

    pub fn min_bit_flips_2220(start: i32, goal: i32) -> i32 {
        let mut k = start ^ goal;
        let mut res = 0;
        while k != 0 {
            k &= k - 1;
            res += 1
        }
        res
    }

    pub fn repeated_character_2351(s: String) -> char {
        let mut mask = 0i32;
        for b in s.as_bytes() {
            let idx = *b - b'a';
            if mask & (1 << idx) == 0 {
                mask |= 1 << idx;
            } else {
                return *b as char;
            }
        }
        return ' ';
    }

    pub fn similar_pairs_2506(words: Vec<String>) -> i32 {
        use std::collections::HashMap;

        let mut freq: HashMap<i32, i32> = HashMap::new();

        words.iter().for_each(|w| {
            let mask = w
                .as_bytes()
                .iter()
                .fold(0i32, |mask, b| mask | (1 << (*b - b'a')));

            freq.entry(mask).and_modify(|c| *c += 1).or_insert(1);
        });

        freq.values().fold(0, |res, cnt| res + cnt * (cnt - 1) / 2)
    }

    pub fn even_odd_bits_2595(mut n: i32) -> Vec<i32> {
        let mut even = 0;
        let mut odd = 0;
        let mut is_even_idx = true;

        while n != 0 {
            if n & 1 == 1 {
                if is_even_idx {
                    even += 1;
                } else {
                    odd += 1;
                }
            }
            is_even_idx = !is_even_idx;
            n >>= 1;
        }

        vec![even, odd]
    }

    pub fn sum_indices_with_k_set_bits_2859(nums: Vec<i32>, k: i32) -> i32 {
        (0..nums.len())
            .filter_map(|i| {
                if i.count_ones() == k as u32 {
                    Some(nums[i])
                } else {
                    None
                }
            })
            .sum()
    }

    pub fn min_operations_2869(nums: Vec<i32>, k: i32) -> i32 {
        let mut mask = 0i64;
        let mut res = 0;
        for n in nums.iter().rev() {
            mask |= 1 << (n - 1);
            res += 1;

            if !mask & ((1 << k) - 1) == 0 {
                return res;
            }
        }

        return res;
    }

    pub fn find_k_or_2917(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let mut cnt = 0;
            for n in nums.iter() {
                if n & (1 << i) != 0 {
                    cnt += 1;
                }

                if cnt == k {
                    res |= 1 << i;
                    break;
                }
            }
        }

        return res;
    }

    pub fn maximum_strong_pair_xor_2932(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() - 1 {
            for j in i + 1..nums.len() {
                if nums[i].abs_diff(nums[j]) as i32 <= nums[i].min(nums[j]) {
                    res = res.max(nums[i] ^ nums[j]);
                }
            }
        }
        return res;
    }

    pub fn has_trailing_zeros_2980(nums: Vec<i32>) -> bool {
        let mut count_even = 0;
        for n in nums.iter() {
            if n & 1 == 0 {
                count_even += 1;
                if count_even == 2 {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn minimum_subarray_length_3095(nums: Vec<i32>, k: i32) -> i32 {
        // A great solution from lc
        let mut res = usize::max_value();
        let mut start = 0;
        let mut end = 0;
        let mut or_val = 0;
        let mut bit_freq = [0u8; 32];

        let do_or = |bit_freq: &mut [u8; 32], or_val: &mut i32, k: i32| {
            *or_val |= k;
            for i in 0..32 {
                if k & (1 << i) != 0 {
                    bit_freq[i] += 1;
                }
            }
        };

        let undo_or = |bit_freq: &mut [u8; 32], or_val: &mut i32, k: i32| {
            for i in 0..32 {
                if k & (1 << i) != 0 {
                    bit_freq[i] -= 1;
                }

                if bit_freq[i] == 0 {
                    *or_val &= !(1 << i);
                }
            }
        };

        while end < nums.len() {
            do_or(&mut bit_freq, &mut or_val, nums[end]);

            if or_val < k {
                end += 1;
                continue;
            }

            while start <= end && or_val >= k {
                undo_or(&mut bit_freq, &mut or_val, nums[start]);
                res = res.min(end - start + 1);
                start += 1;
            }

            end += 1;
        }

        if res == usize::max_value() {
            return -1;
        }

        res as i32
    }

    pub fn duplicate_numbers_xor_3158(nums: Vec<i32>) -> i32 {
        let mut used = 0i64;

        nums.iter().fold(0, |mut res, n| {
            if used & (1 << (n - 1)) == 0 {
                used |= 1 << (n - 1);
            } else {
                res ^= n;
            }

            return res;
        })
    }

    pub fn min_changes_3226(n: i32, k: i32) -> i32 {
        let diff = n ^ k;
        let k = n & diff;

        if k.count_ones() != diff.count_ones() {
            return -1;
        }

        diff.count_ones() as i32
    }

    pub fn kth_character_3304(k: i32) -> char {
        // don't understand solution (fuck Lee)
        (b'a' + (k - 1).count_ones() as u8) as char
    }

    pub fn min_bitwise_array_3314(nums: Vec<i32>) -> Vec<i32> {
        // k | (k + 1) = (..10111) | (..11000) = (..11111)
        // This operation sets the lowest 0 bit to 1
        nums.iter()
            .map(|n| {
                if n & 1 == 0 {
                    -1
                } else {
                    n & !(((n + 1) & -(n + 1)) >> 1)
                }
            })
            .collect()
    }

    pub fn smallest_number_3370(n: i32) -> i32 {
        let mut k = 1;
        while k <= n {
            k <<= 1;
        }
        return k - 1;
    }
}
