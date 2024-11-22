struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn delete_duplicates_83(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = &mut head;
        while let Some(node) = curr {
            while let Some(next) = &mut node.next {
                if node.val == next.val {
                    node.next = next.next.take()
                } else {
                    break;
                }
            }
            curr = &mut node.next;
        }
        head
    }

    pub fn convert_to_title_168(column_number: i32) -> String {
        let mut res = String::new();
        let mut col_num = column_number;
        while col_num > 0 {
            col_num -= 1;
            let char_code = ((col_num % 26) as u8) + b'A';
            res.insert(0, char_code as char);
            col_num /= 26;
        }
        res
    }

    pub fn title_to_number_171(column_title: String) -> i32 {
        let mut res: i32 = 0;
        for c in column_title.bytes() {
            res *= 26;
            res += (c as i32 - 65) + 1;
        }
        res
    }

    pub fn is_isomorphic_205_1(s: String, t: String) -> bool {
        let mut char_index_s: std::collections::HashMap<char, usize> =
            std::collections::HashMap::new();
        let mut char_index_t: std::collections::HashMap<char, usize> =
            std::collections::HashMap::new();

        for (i, (ch_s, ch_t)) in s.chars().zip(t.chars()).enumerate() {
            char_index_s.entry(ch_s).or_insert(i);
            char_index_t.entry(ch_t).or_insert(i);
            if char_index_s[&ch_s] != char_index_t[&ch_t] {
                return false;
            }
        }

        true
    }

    pub fn is_isomorphic_205_2(a: String, b: String) -> bool {
        let a = a.as_bytes();
        let b = b.as_bytes();

        if a.len() > 65535 || b.len() > 65535 || a.len() != b.len() {
            return false;
        }

        let mut m1 = [0u16; 128];
        let mut m2 = [0u16; 128];

        for (i, j) in std::iter::zip(a, b).enumerate() {
            let c = &mut m1[(*j.0 & 127) as usize];
            let d = &mut m2[(*j.1 & 127) as usize];

            if *c != *d {
                return false;
            }

            *c = i as u16 + 1;
            *d = i as u16 + 1;
        }

        true
    }

    pub fn to_lower_case_709(s: String) -> String {
        s.to_lowercase()
    }

    pub fn is_anagram_242_1(s: String, t: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();

        s_chars.sort();
        t_chars.sort();

        let s: String = s_chars.into_iter().collect();
        let t: String = t_chars.into_iter().collect();

        s == t
    }

    pub fn is_anagram_242_2(s: String, t: String) -> bool {
        let mut count: std::collections::HashMap<char, i32> = std::collections::HashMap::new();
        for c in s.chars() {
            *count.entry(c).or_insert(0) += 1
        }
        for c in t.chars() {
            *count.entry(c).or_insert(0) -= 1
        }
        count.values().all(|&val| val == 0)
    }

    pub fn is_anagram_242_3(s: String, t: String) -> bool {
        let mut count: [i32; 26] = [0; 26];
        // count the frequency of characters in string s
        for c in s.chars() {
            count[(c as u8 - b'a') as usize] += 1;
        }
        // decrement the frequency of characters in string t
        for c in t.chars() {
            count[(c as u8 - b'a') as usize] -= 1;
        }
        // check if any character has non-zero frequency
        for val in count {
            if val != 0 {
                return false;
            }
        }
        true
    }
}
