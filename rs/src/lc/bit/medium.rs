#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn divide_29(dividend: i32, divisor: i32) -> i32 {
        let is_neg = (dividend < 0) ^ (divisor < 0);

        let mut p = if dividend > 0 { -dividend } else { dividend };
        let q = if divisor > 0 { -divisor } else { divisor };

        let mut res = 0;

        for k in (0..q.leading_ones()).rev() {
            if p <= (q << k) {
                p -= q << k;
                res += -1 << k;
            }
        }

        if is_neg {
            res
        } else if res == i32::min_value() {
            i32::max_value()
        } else {
            -res
        }
    }

    pub fn single_number_137_1(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut i = 0;

        while i < nums.len() {
            if i == nums.len() - 1
                || !(nums[i] ^ nums[i + 1] == 0 && nums[i + 1] ^ nums[i + 2] == 0)
            {
                return nums[i];
            }
            i += 3;
        }

        return 0;
    }

    pub fn single_number_137_2(nums: Vec<i32>) -> i32 {
        let (mut x1, mut x2, mut mask) = (0, 0, 0);

        nums.iter().for_each(|k| {
            x2 ^= x1 & k;
            x1 ^= k;

            mask = !(x1 & x2);
            x2 &= mask;
            x1 &= mask;
        });

        return x1;
    }

    pub fn single_number_137_3(nums: Vec<i32>) -> i32 {
        let mut once = 0;
        let mut twice = 0;

        nums.iter().for_each(|k| {
            once = (once ^ k) & (!twice);
            twice = (twice ^ k) & (!once);
        });

        return once;
    }

    pub fn find_repeated_dna_sequences_187(s: String) -> Vec<String> {
        use std::collections::HashMap;
        // The problem can be solved using the sliding window technique while storing 10-character-long substrings as keys in a hash map.
        // However, the key challenge here is optimizing memory usage for these keys. To achieve this, we use a rolling hash.
        // Since each of the four characters can be uniquely identified using only their last three bits, we can efficiently encode them.
        // Another approach is to map each character to a distinct bit pair, rather than relying on their actual binary representation

        let mut freq: HashMap<u32, u32> = HashMap::new();
        let mut encoded_str: u32 = 0;
        let mut res = vec![];

        for i in 0..s.len() {
            let new_char = s.as_bytes()[i] as u32 & 7;
            encoded_str = (encoded_str << 3) & 0x3fffffff;
            encoded_str |= new_char;

            if i >= 9 {
                let count = freq.entry(encoded_str).or_insert(0);
                *count += 1;
                if *count == 2 {
                    res.push(s[i - 9..=i].to_string());
                }
            }
        }

        res
    }

    pub fn range_bitwise_and_201_1(left: i32, right: i32) -> i32 {
        if left == right {
            return left;
        }

        let msb = |mut k: i32| -> i32 {
            k |= k >> 1;
            k |= k >> 2;
            k |= k >> 4;
            k |= k >> 8;
            k |= k >> 16;
            return k;
        };

        let mut mask = msb(left);

        if mask != i32::max_value() && right >= mask + 1 {
            return 0;
        }

        mask = msb(left ^ right);
        left & !mask
    }

    pub fn range_bitwise_and_201_2(left: i32, mut right: i32) -> i32 {
        while right > left {
            right &= right - 1;
        }
        left & right
    }

    pub fn single_number_260(nums: Vec<i32>) -> Vec<i32> {
        let xor_acc = nums.iter().fold(0, |acc, n| acc ^ *n);

        let mask = xor_acc & -xor_acc; // get lsb of xor_acc
        let first = nums.iter().fold(0, |acc, n| {
            if *n & mask == 0 {
                return acc ^ *n;
            }
            return acc;
        });

        vec![first, xor_acc ^ first]
    }

    pub fn find_duplicate_287(nums: Vec<i32>) -> i32 {
        let n = nums.len() - 1;
        let mut k = n;
        k |= k >> 1;
        k |= k >> 2;
        k |= k >> 4;
        k |= k >> 8;
        k |= k >> 16;

        let bit_len = k.count_ones();

        let mut res = 0;
        for i in 0..bit_len {
            let mut x = 0;
            let mut y = 0;
            for k in 0..nums.len() {
                if nums[k] & (1 << i) != 0 {
                    x += 1;
                }

                if k > 0 && (k & (1 << i) != 0) {
                    y += 1;
                }
            }

            if x > y {
                res |= 1 << i;
            }
        }
        res
    }

    pub fn max_product_318(words: Vec<String>) -> i32 {
        let words_bit: Vec<u32> = words
            .iter()
            .map(|s| {
                s.as_bytes()
                    .iter()
                    .fold(0u32, |mask, b| mask | (1 << (*b - b'a')))
            })
            .collect();

        let mut res = 0;
        for i in 0..words.len() - 1 {
            for j in 1..words.len() {
                if words_bit[i] & words_bit[j] == 0 {
                    res = res.max(words[i].len() * words[j].len());
                }
            }
        }
        res as i32
    }

    pub fn get_sum_371(a: i32, b: i32) -> i32 {
        // a super complicated solution :(((
        let bit_len = |mut k: i32| -> u8 {
            k |= k >> 1;
            k |= k >> 2;
            k |= k >> 4;
            k |= k >> 8;
            k |= k >> 16;
            k.count_ones() as u8
        };

        let calc = |mut a: i32, mut b: i32, is_add: bool| -> i32 {
            if a < b {
                (b, a) = (a, b);
            }

            let a_len = bit_len(a);
            let b_len = bit_len(b);

            let mut idx = 0;
            let mut res = 0;
            let mut rem = false;
            while idx < a_len {
                if idx < b_len {
                    let cur_a = a & (1 << idx);
                    let cur_b = b & (1 << idx);

                    res |= if rem {
                        cur_a ^ cur_b ^ (1 << idx)
                    } else {
                        cur_a ^ cur_b
                    };

                    rem = if is_add {
                        (cur_a & cur_b != 0) || (cur_a ^ cur_b != 0 && rem)
                    } else {
                        (cur_a == 0 && cur_b != 0) || (cur_a ^ cur_b == 0 && rem)
                    };
                } else {
                    let cur_a = a & (1 << idx);
                    res |= if rem { cur_a ^ (1 << idx) } else { cur_a };
                    rem &= if is_add { cur_a != 0 } else { cur_a == 0 };
                }
                idx += 1;
            }

            if rem && is_add {
                res |= 1 << idx;
            }

            res
        };

        let a_abs = a.abs();
        let b_abs = b.abs();

        if a >= 0 && b >= 0 {
            calc(a, b, true)
        } else if a < 0 && b < 0 {
            !calc(a_abs, b_abs, true) + 1
        } else {
            let res = calc(a_abs, b_abs, false);
            if (a >= 0 && a < b_abs) || (b >= 0 && b < a_abs) {
                !res + 1
            } else {
                res
            }
        }
    }

    pub fn get_sum_371_2(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let carry = a & b;
            a ^= b;
            b = carry << 1;
        }
        return a;
    }

    pub fn valid_utf8_393(data: Vec<i32>) -> bool {
        let mut count = 0;
        for c in data.iter() {
            if count == 0 {
                if c >> 5 == 0b110 {
                    count = 1;
                } else if c >> 4 == 0b1110 {
                    count = 2;
                } else if c >> 3 == 0b11110 {
                    count = 3;
                } else if c >> 7 == 1 {
                    return false;
                }
            } else {
                if c >> 6 != 0b10 {
                    return false;
                }
                count -= 1;
            }
        }
        count == 0
    }

    pub fn circular_permutation_1238(n: i32, start: i32) -> Vec<i32> {
        (0..1 << n).fold(vec![], |mut res, k| {
            res.push(start ^ k ^ (k >> 1));
            return res;
        })
    }

    pub fn max_length_1239(arr: Vec<String>) -> i32 {
        let masks: Vec<i32> = arr
            .iter()
            .filter_map(|s| {
                let mut mask = 0i32;
                for b in s.as_bytes() {
                    if mask & (1 << (*b - b'a')) != 0 {
                        return None;
                    } else {
                        mask |= 1 << (*b - b'a');
                    }
                }

                return Some(mask);
            })
            .collect();

        fn solve(idx: usize, acc: i32, res: &mut u32, masks: &Vec<i32>) {
            for i in idx..masks.len() {
                if acc ^ masks[i] == acc | masks[i] {
                    solve(i + 1, acc ^ masks[i], res, masks);
                }
            }

            *res = acc.count_ones().max(*res);
        }

        let mut res = 0;
        solve(0, 0, &mut res, &masks);
        res as i32
    }
}
