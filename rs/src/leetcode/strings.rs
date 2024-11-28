#[allow(dead_code)]
pub struct Solution;

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
        // bad code
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

    pub fn find_the_difference_389_1(s: String, t: String) -> char {
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();

        s_chars.sort();
        t_chars.sort();

        for i in 0..s_chars.len() {
            if s_chars[i] != t_chars[i] {
                return t_chars[i];
            }
        }

        t_chars[t_chars.len() - 1]
    }

    pub fn find_the_difference_389_2(s: String, t: String) -> char {
        let mut freqs: std::collections::HashMap<char, i32> = std::collections::HashMap::new();

        for c in s.chars() {
            freqs.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }

        let mut added_char = ' ';
        for c in t.chars() {
            match freqs.get(&c) {
                None | Some(0) => added_char = c,
                Some(&f) => {
                    freqs.insert(c, f - 1);
                }
            }
        }

        added_char
    }

    pub fn longest_palindrome_409(s: String) -> i32 {
        let mut count: std::collections::HashMap<char, i16> = std::collections::HashMap::new();

        for c in s.chars() {
            count.entry(c).and_modify(|freq| *freq += 1).or_insert(1);
        }

        let mut res: i32 = 0;
        let mut flag = false;
        for v in count.values() {
            if *v % 2 == 1 {
                res += (*v - 1) as i32;
                flag = true;
                continue;
            }

            res += *v as i32;
        }

        if flag {
            return res + 1;
        }

        res
    }

    pub fn fizz_buzz_412(n: i32) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for i in 1..=n {
            match i {
                _ if i % 3 == 0 && i % 5 == 0 => res.push(String::from("FizzBuzz")),
                _ if i % 3 == 0 => res.push(String::from("Fizz")),
                _ if i % 5 == 0 => res.push(String::from("Buzz")),
                _ => res.push(i.to_string()),
            }
        }
        res
    }

    pub fn add_strings_415(num1: String, num2: String) -> String {
        let mut m = num1.len();
        let mut n = num2.len();

        let num1_chars: Vec<char> = num1.chars().collect();
        let num2_chars: Vec<char> = num2.chars().collect();

        let mut rem = false;
        let mut res: Vec<char> = vec![];

        while m > 0 || n > 0 {
            let mut digit: u8;

            match (m, n) {
                (_, _) if m > 0 && n > 0 => {
                    m -= 1;
                    n -= 1;
                    digit = (num1_chars[m] as u8 - '0' as u8) + (num2_chars[n] as u8 - '0' as u8);
                }
                (_, _) if m > 0 => {
                    m -= 1;
                    digit = num1_chars[m] as u8 - '0' as u8;
                }
                (_, _) if n > 0 => {
                    n -= 1;
                    digit = num2_chars[n] as u8 - '0' as u8;
                }
                (_, _) => break,
            }

            if rem {
                digit += 1
            }

            rem = digit >= 10;

            res.push(((digit % 10) as u8 + b'0') as char);
        }

        if rem {
            res.push('1');
        }

        res.reverse();
        res.into_iter().collect()
    }

    pub fn count_segments_434(s: String) -> i32 {
        s.split_whitespace().count() as i32
    }

    pub fn repeated_substring_pattern_459_1(s: String) -> bool {
        let n = s.len();
        for i in 1..=n / 2 {
            if n % i == 0 {
                let substr = &s[0..i];
                let repeated = substr.repeat(n / i);
                if repeated == s {
                    return true;
                }
            }
        }

        false
    }

    pub fn repeated_substring_pattern_459_2(s: String) -> bool {
        let doubled = s.clone() + &s;
        let sub = &doubled[1..doubled.len() - 1];
        sub.contains(&s)
    }

    pub fn license_key_formatting_482(s: String, k: i32) -> String {
        let mut res: Vec<char> = vec![];
        let mut i = 0;
        for c in s.chars().rev() {
            if c == '-' {
                continue;
            }

            if i == k {
                i = 0;
                res.push('-');
            }

            res.push(c.to_ascii_uppercase());
            i += 1;
        }

        res.reverse();
        res.into_iter().collect()
    }

    pub fn find_words_500(words: Vec<String>) -> Vec<String> {
        let rows: [std::collections::HashSet<char>; 3] = [
            "qwertyuiop".chars().collect(),
            "asdfghjkl".chars().collect(),
            "zxcvbnm".chars().collect(),
        ];

        words
            .into_iter()
            .filter(|w| {
                let chars: std::collections::HashSet<char> = w.to_lowercase().chars().collect();
                rows.iter().any(|row| chars.is_subset(row))
            })
            .collect()
    }

    pub fn detect_capital_use_520(word: String) -> bool {
        let n = word.len();
        let chars: Vec<char> = word.chars().collect();
        let first_char_is_uppercase = chars[0].is_ascii_uppercase();

        let mut uppercase_count = 0;
        if first_char_is_uppercase {
            uppercase_count += 1;
        }

        for i in 1..n {
            if chars[i].is_ascii_uppercase() {
                uppercase_count += 1;
            }
        }

        uppercase_count == 0
            || (uppercase_count == 1 && first_char_is_uppercase)
            || (uppercase_count == n)
    }

    pub fn find_lus_length_521(a: String, b: String) -> i32 {
        if a == b {
            return -1;
        }

        std::cmp::max(a.len(), b.len()) as i32
    }

    pub fn reverse_str_541(s: String, k: i32) -> String {
        let k: usize = k as usize;

        s.chars()
            .collect::<Vec<char>>()
            .chunks(2 * k)
            .map(|chunk| {
                let substr_len = chunk.len();
                if k >= substr_len {
                    return chunk.iter().rev().collect::<String>();
                }

                chunk[..k].iter().rev().collect::<String>() + &chunk[k..].iter().collect::<String>()
            })
            .collect::<String>()
    }

    pub fn check_record_551(s: String) -> bool {
        let mut absent_count = 0;

        if s.contains("LLL") {
            return false;
        }

        for c in s.chars() {
            if c == 'A' {
                absent_count += 1;
                if absent_count >= 2 {
                    return false;
                }
            }
        }

        true
    }

    pub fn reverse_words_557_1(s: String) -> String {
        s.split_whitespace()
            .map(|word| word.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn reverse_words_557_2(s: String) -> String {
        let mut bytes = s.into_bytes();
        let n = bytes.len();

        let mut prev_end = 0;
        for i in 0..n {
            if bytes[i] == b' ' {
                bytes[prev_end..i].reverse();
                prev_end = i + 1;
            }
        }
        bytes[prev_end..n].reverse();
        unsafe { String::from_utf8_unchecked(bytes) }
    }

    pub fn find_restaurant_599_1(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        // bad code
        let m = list1.len();
        let n = list2.len();

        let mut least_index_sum = usize::MAX;
        let mut res: Vec<String> = vec![];

        for i in 0..m {
            for j in 0..n {
                if list1[i] == list2[j] {
                    if i + j < least_index_sum {
                        res.clear();
                        least_index_sum = i + j;
                        res.push(list1[i].clone());
                        continue;
                    }

                    if i + j == least_index_sum {
                        res.push(list1[i].clone());
                    }
                }
            }
        }

        res
    }

    pub fn find_restaurant_599_2(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let map2: std::collections::HashMap<String, usize> = list2
            .iter()
            .enumerate()
            .map(|(idx, str)| (str.to_string(), idx))
            .collect();

        let mut least_index_sum = usize::MAX;
        let mut res: Vec<String> = vec![];

        for (i, str) in list1.iter().enumerate() {
            if let Some(j) = map2.get(str) {
                if i + j < least_index_sum {
                    res.clear();
                    least_index_sum = i + j;
                    res.push(str.to_string());
                    continue;
                }

                if i + j == least_index_sum {
                    res.push(str.to_string());
                }
            }
        }

        res
    }

    pub fn judge_circle_657_1(moves: String) -> bool {
        // bad code
        let mut moves_map: std::collections::HashMap<char, u16> = std::collections::HashMap::new();
        for c in moves.chars() {
            *moves_map.entry(c).or_insert(1) += 1;
        }

        match (
            moves_map.get(&'R'),
            moves_map.get(&'L'),
            moves_map.get(&'U'),
            moves_map.get(&'D'),
        ) {
            (Some(r), Some(l), _, _) if r != l => false,
            (Some(_), None, _, _) => false,
            (None, Some(_), _, _) => false,
            (_, _, Some(u), Some(d)) if u != d => false,
            (_, _, Some(_), None) => false,
            (_, _, None, Some(_)) => false,
            (_, _, _, _) => true,
        }
    }

    pub fn judge_circle_657_2(moves: String) -> bool {
        let mut state = (0i32, 0i32);
        for c in moves.chars() {
            match c {
                'R' => state.0 += 1,
                'L' => state.0 -= 1,
                'U' => state.1 += 1,
                'D' => state.1 -= 1,
                _ => {
                    continue;
                }
            }
        }
        state == (0i32, 0i32)
    }
}
