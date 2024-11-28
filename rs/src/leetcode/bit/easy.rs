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
}
