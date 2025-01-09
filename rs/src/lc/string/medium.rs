#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn int_to_roman_12(mut num: i32) -> String {
        let mp: [char; 7] = ['M', 'D', 'C', 'L', 'X', 'V', 'I'];

        let mut res = String::new();
        let mut k = 1000;
        let mut i = 0;
        while k != 0 {
            if num / k != 0 {
                let digit = num / k;
                if digit < 4 {
                    res.push_str(&mp[i].to_string().repeat(digit as usize));
                } else if 4 < digit && digit < 9 {
                    res.push(mp[i - 1]);
                    res.push_str(&mp[i].to_string().repeat(digit as usize - 5));
                } else if digit == 4 {
                    res.push(mp[i]);
                    res.push(mp[i - 1]);
                } else if digit == 9 {
                    res.push(mp[i]);
                    res.push(mp[i - 2]);
                }

                num = num % k;
            } else {
                k /= 10;
                i += 2;
            }
        }

        res
    }

    pub fn count_and_say_38(n: i32) -> String {
        fn rec(s: String, n: i32) -> String {
            if n == 1 {
                return s;
            }

            let mut res = String::new();
            let mut prev = s.as_bytes()[0] as char;
            let mut cnt = 0;

            s.chars().for_each(|c| {
                if c == prev {
                    cnt += 1;
                } else {
                    res.push_str(&format!("{}{}", cnt, prev));
                    cnt = 1;
                    prev = c;
                }
            });

            res.push_str(&format!("{}{}", cnt, prev));
            rec(res, n - 1)
        }

        rec(String::from("1"), n)
    }

    pub fn multiply_43_1(num1: String, num2: String) -> String {
        let mul = |s: &str, digit: u8, n: usize| -> String {
            let mut bytes: Vec<u8> = vec![];
            let mut carry = 0;
            s.as_bytes().iter().rev().for_each(|b| {
                let k = (*b - b'0') * digit + carry;
                bytes.push(k % 10 + b'0');
                carry = k / 10;
            });
            while carry > 0 {
                bytes.push((carry % 10) + b'0');
                carry /= 10;
            }
            bytes.reverse();
            let mut res = unsafe { String::from_utf8_unchecked(bytes) + &"0".repeat(n as usize) };
            res = res.trim_start_matches('0').to_string();
            if res.len() == 0 {
                return String::from("0");
            }
            res
        };

        let add = |s1: String, s2: String| -> String {
            let mut res: Vec<u8> = vec![];
            let mut i = s1.len();
            let mut j = s2.len();
            let mut rem = 0;
            while i > 0 || j > 0 {
                let k: u8;
                if i > 0 && j > 0 {
                    i -= 1;
                    j -= 1;
                    k = s1.as_bytes()[i] + s2.as_bytes()[j] - 2 * b'0' + rem;
                } else if i > 0 {
                    i -= 1;
                    k = s1.as_bytes()[i] - b'0' + rem;
                } else {
                    j -= 1;
                    k = s2.as_bytes()[j] - b'0' + rem;
                }

                res.push(k % 10 + b'0');
                rem = k / 10;
            }

            if rem != 0 {
                res.push(rem + b'0');
            }

            res.reverse();
            unsafe { String::from_utf8_unchecked(res) }
        };

        (0..num1.len()).fold(String::from("0"), |res, i| {
            add(
                res,
                mul(&num2, num1.as_bytes()[i] - b'0', num1.len() - 1 - i),
            )
        })
    }

    pub fn multiply_43_2(num1: String, num2: String) -> String {
        let mut bytes: Vec<u8> = vec![0u8; num1.len() + num2.len()];
        (0..num1.len()).rev().for_each(|i| {
            (0..num2.len()).rev().for_each(|j| {
                let d = (num1.as_bytes()[i] - b'0') * (num2.as_bytes()[j] - b'0');
                let (p1, p2) = (i + j, i + j + 1);
                let k = d + bytes[p2];

                bytes[p1] += k / 10;
                bytes[p2] = k % 10;
            });
        });

        let res = unsafe {
            String::from_utf8_unchecked(bytes.iter().fold(vec![], |mut res: Vec<u8>, b| {
                if !(res.len() == 0 && *b == 0) {
                    res.push(*b + b'0');
                }
                res
            }))
        };

        if res.len() == 0 {
            return "0".to_string();
        }
        res
    }

    pub fn group_anagrams_49(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        strs.iter()
            .fold(
                HashMap::new(),
                |mut acc: HashMap<String, Vec<String>>, s| {
                    let mut chars: Vec<char> = s.chars().collect();
                    chars.sort();
                    let key: String = chars.into_iter().collect();
                    acc.entry(key.clone())
                        .and_modify(|v| v.push(s.clone()))
                        .or_insert(vec![s.clone()]);
                    acc
                },
            )
            .into_values()
            .collect()
    }

    pub fn simplify_path_71(path: String) -> String {
        "/".to_string()
            + &path
                .split('/')
                .fold(vec![], |mut acc: Vec<String>, s: &str| {
                    if !s.is_empty() && s != "." {
                        if s == ".." {
                            acc.pop();
                        } else {
                            acc.push(s.to_string());
                        }
                    }
                    acc
                })
                .join("/")
    }

    pub fn num_decodings_91_1(s: String) -> i32 {
        // Recursion -> TLE :(
        fn decode(idx: usize, s: &str) -> i32 {
            if idx == s.len() {
                return 1;
            }
            if s.as_bytes()[idx] == b'0' {
                return 0;
            }
            let mut res = decode(idx + 1, s);
            if idx < s.len() - 1
                && (s.as_bytes()[idx] == b'1'
                    || (s.as_bytes()[idx] == b'2' && s.as_bytes()[idx + 1] < b'7'))
            {
                res += decode(idx + 2, s);
            }
            return res;
        }

        if s.is_empty() {
            0
        } else {
            decode(0, &s)
        }
    }

    pub fn num_decodings_91_2(s: String) -> i32 {
        // DP: tabulation
        let mut dp = vec![0; s.len() + 1];
        dp[s.len()] = 1;
        (0..s.len()).rev().for_each(|i| {
            if s.as_bytes()[i] == b'0' {
                dp[i] = 0;
            } else {
                dp[i] = dp[i + 1];
                if i < s.len() - 1
                    && (s.as_bytes()[i] == b'1'
                        || (s.as_bytes()[i] == b'2' && s.as_bytes()[i + 1] < b'7'))
                {
                    dp[i] += dp[i + 2];
                }
            }
        });
        dp[0]
    }

    pub fn num_decodings_91_3(s: String) -> i32 {
        // DP: constant space
        let mut res = 1;
        let mut prev = 0;
        for i in (0..s.len()).rev() {
            let mut curr = if s.as_bytes()[i] == b'0' { 0 } else { res };

            if i < s.len() - 1
                && (s.as_bytes()[i] == b'1'
                    || (s.as_bytes()[i] == b'2' && s.as_bytes()[i + 1] < b'7'))
            {
                curr += prev;
            }
            prev = res;
            res = curr;
        }
        res
    }

    pub fn restore_ip_addresses_93(_s: String) -> Vec<String> {
        vec![]
    }

    pub fn partition_131(_s: String) -> Vec<Vec<String>> {
        vec![]
    }

    pub fn word_break_139(_s: String, _word_dict: Vec<String>) -> bool {
        false
    }

    pub fn reverse_words_151(_s: String) -> String {
        String::new()
    }
}
