#[allow(dead_code)]
struct Solution;

#[allow(dead_code)]
impl Solution {
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

    pub fn word_pattern_290(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split_whitespace().collect();
        if pattern.len() != words.len() {
            return false;
        }

        let mut pattern_to_word: std::collections::HashMap<char, &str> =
            std::collections::HashMap::new();
        let mut word_to_pattern: std::collections::HashMap<&str, char> =
            std::collections::HashMap::new();

        for (c, w) in pattern.chars().zip(words.iter()) {
            if let Some(mapped_word) = pattern_to_word.get(&c) {
                if mapped_word != w {
                    return false;
                }
            } else {
                pattern_to_word.insert(c, w);
            }

            if let Some(mapped_char) = word_to_pattern.get(w) {
                if mapped_char != &c {
                    return false;
                }
            } else {
                word_to_pattern.insert(w, c);
            }
        }

        true
    }

    pub fn reverse_string_344(s: &mut Vec<char>) {
        let n = s.len();
        for i in 0..n / 2 {
            s.swap(i, n - 1 - i);
        }
    }

    pub fn reverse_vowels_345_1(s: String) -> String {
        const VOWELS: &str = "aeiouAEIOU";

        let mut chars: Vec<char> = s.chars().collect();
        let mut n = chars.len();
        let mut index_vowels = vec![];

        for i in 0..n {
            if VOWELS.contains(chars[i]) {
                index_vowels.push(i);
            }
        }

        n = index_vowels.len();
        for i in 0..n / 2 {
            chars.swap(index_vowels[i], index_vowels[n - 1 - i]);
        }

        chars.iter().collect()
    }

    pub fn reverse_vowels_345_2(s: String) -> String {
        let mut vowels = vec![];
        let mut positions = vec![];
        let mut i = 0;

        for each in s.chars() {
            let c = each.to_ascii_lowercase();
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                vowels.push(each);
                positions.push(i);
            }
            i += 1;
        }
        vowels.reverse();

        let mut res = s;
        i = 0;
        for each in positions {
            res.replace_range(each..each + 1, vowels[i].to_string().as_str());
            i += 1;
        }

        res
    }

    pub fn can_construct_383_1(ransom_note: String, magazine: String) -> bool {
        let mut ran_chars: std::collections::HashMap<char, i16> = std::collections::HashMap::new();
        let mut mag_chars: std::collections::HashMap<char, i16> = std::collections::HashMap::new();

        for c in ransom_note.chars() {
            *ran_chars.entry(c).or_insert(0) += 1;
        }

        for c in magazine.chars() {
            *mag_chars.entry(c).or_insert(0) += 1;
        }

        for c in ran_chars.keys() {
            if let Some(val) = mag_chars.get(c) {
                if ran_chars.get(c).unwrap() > val {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    pub fn can_construct_383_2(ransom_note: String, magazine: String) -> bool {
        let mut magazine_counts = [0; 26];

        for c in magazine.chars() {
            magazine_counts[c as usize - 'a' as usize] += 1;
        }

        for c in ransom_note.chars() {
            let index = c as usize - 'a' as usize;
            if magazine_counts[index] == 0 {
                return false;
            }
            magazine_counts[index] -= 1;
        }
        true
    }

    pub fn first_uniq_char_387_1(s: String) -> i32 {
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        if n == 1 {
            return 0;
        }

        for i in 0..n {
            for j in 0..n {
                if i == n - 1 && j == n - 1 {
                    return i as i32;
                }

                if i == j {
                    continue;
                }

                if chars[i] == chars[j] {
                    break;
                }

                if j == n - 1 {
                    return i as i32;
                }
            }
        }

        -1
    }

    pub fn first_uniq_char_387_2(s: String) -> i32 {
        let mut first_unique = i32::MAX;

        for c in 'a'..='z' {
            if s.contains(c) {
                if s.find(c) == s.rfind(c) {
                    first_unique = first_unique.min(s.find(c).unwrap() as i32);
                }
            }
        }

        if first_unique == i32::MAX {
            -1
        } else {
            first_unique
        }
    }
}
