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
        use std::collections::HashMap;
        let mut char_index_s: HashMap<char, usize> = HashMap::new();
        let mut char_index_t: HashMap<char, usize> = HashMap::new();

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
        use std::collections::HashMap;
        let mut count: HashMap<char, i32> = HashMap::new();
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
        use std::collections::HashMap;

        let words: Vec<&str> = s.split_whitespace().collect();
        if pattern.len() != words.len() {
            return false;
        }

        let mut pattern_to_word: HashMap<char, &str> = HashMap::new();
        let mut word_to_pattern: HashMap<&str, char> = HashMap::new();

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
        use std::collections::HashMap;

        let mut ran_chars: HashMap<char, i16> = HashMap::new();
        let mut mag_chars: HashMap<char, i16> = HashMap::new();

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
        use std::collections::HashMap;
        let mut freqs: HashMap<char, i32> = HashMap::new();

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
        use std::collections::HashMap;
        let mut count: HashMap<char, i16> = HashMap::new();

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
        use std::collections::HashSet;
        let rows: [HashSet<char>; 3] = [
            "qwertyuiop".chars().collect(),
            "asdfghjkl".chars().collect(),
            "zxcvbnm".chars().collect(),
        ];

        words
            .into_iter()
            .filter(|w| {
                let chars: HashSet<char> = w.to_lowercase().chars().collect();
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
        use std::collections::HashMap;
        let mut moves_map: HashMap<char, u16> = HashMap::new();
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

    pub fn valid_palindrome_680_1(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let is_palindrome = |s: &[char], mut i: usize, mut j: usize| -> bool {
            while i < j {
                if s[i] != s[j] {
                    return false;
                }
                i += 1;
                j -= 1;
            }
            true
        };

        let mut i = 0usize;
        let mut j = s.len() - 1;

        while i < j {
            if chars[i] != chars[j] {
                return is_palindrome(&chars, i + 1, j) || is_palindrome(&chars, i, j - 1);
            }
            i += 1;
            j -= 1;
        }

        true
    }

    pub fn valid_palindrome_680_2(s: String) -> bool {
        let s: &[u8] = s.as_bytes();
        let is_palindrome =
            |s: &[u8]| -> bool { (0..s.len() / 2).all(|i| s[i] == s[s.len() - 1 - i]) };

        for i in 0..s.len() / 2 {
            if s[i] != s[s.len() - 1 - i] {
                return s[i + 1] == s[s.len() - 1 - i] && is_palindrome(&s[i + 1..s.len() - i])
                    || s[i] == s[s.len() - 2 - i] && is_palindrome(&s[i..s.len() - i - 1]);
            }
        }

        true
    }

    pub fn count_binary_substrings_696(s: String) -> i32 {
        // previous continuous occurrence, current continuous occurrence
        let mut prev = 0;
        let mut curr = 1;

        // counter for binary substrings with equal 0s and 1s
        let mut count = 0;

        let chars = s.chars().collect::<Vec<char>>();

        for i in 1..s.len() {
            if chars[i] == chars[i - 1] {
                curr += 1;
                continue;
            }

            count += prev.min(curr);
            prev = curr;
            curr = 1;
        }
        count + prev.min(curr)
    }

    pub fn shortest_completing_word_748_1(license_plate: String, words: Vec<String>) -> String {
        // bad code but beast 100%, lmao :))
        let mut freq_lp = [0; 26];
        license_plate.chars().for_each(|c| {
            if c.is_ascii_alphabetic() {
                freq_lp[((c.to_ascii_lowercase() as u8) - b'a') as usize] += 1;
            }
        });

        words
            .into_iter()
            .filter(|w| {
                let mut freq_w = [0; 26];
                w.chars()
                    .for_each(|c| freq_w[((c as u8) - b'a') as usize] += 1);
                freq_w.iter().zip(freq_lp.iter()).all(|(w, p)| w >= p)
            })
            .min_by_key(String::len)
            .unwrap()
    }

    pub fn shortest_completing_word_748_2(license_plate: String, words: Vec<String>) -> String {
        // super code (from Solutions) -> beast 66.67%, wtf ???
        let s = String::from("aaaaaaaaaaaaaaaaaaaa");
        let primes: [usize; 27] = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103,
        ];

        let get_char_prod = |plate: &String| -> usize {
            let mut prod: usize = 1;
            for c in plate.as_bytes() {
                let index: u8 = c - b'a';
                if index <= 25 {
                    prod *= primes[index as usize]
                }
            }
            prod
        };

        let char_prod = get_char_prod(&license_plate.to_ascii_lowercase());
        let mut res: &String = &s;

        for w in &words {
            if w.len() < res.len() && get_char_prod(&w) % char_prod == 0 {
                res = &w;
            }
        }

        res.to_string()
    }

    pub fn num_jewels_in_stones_771(jewels: String, stones: String) -> i32 {
        let mut freq = [0u8; 52];
        let mut res = 0;

        for c in stones.as_bytes() {
            if *c >= b'a' {
                freq[(*c - b'a' + 26) as usize] += 1;
                continue;
            }

            freq[(*c - b'A') as usize] += 1;
        }

        for c in jewels.as_bytes() {
            if *c >= b'a' {
                res += freq[(*c - b'a' + 26) as usize];
                continue;
            }

            res += freq[(*c - b'A') as usize];
        }

        res as i32
    }

    pub fn rotate_string_796(s: String, goal: String) -> bool {
        let n = s.len();
        let mut chars: Vec<char> = s.chars().collect();
        let mut i = 0;

        while i <= n {
            chars.push(chars[0]);
            chars.remove(0);
            if chars.iter().collect::<String>() == goal {
                return true;
            }
            i += 1;
        }

        false
    }

    pub fn unique_morse_representations_804(words: Vec<String>) -> i32 {
        const ALPHABET_MORSES: [&str; 26] = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];

        let res_set: std::collections::HashSet<String> = words
            .iter()
            .map(|w| {
                w.as_bytes()
                    .iter()
                    .map(|&c| ALPHABET_MORSES[(c - b'a') as usize])
                    .collect::<String>()
            })
            .collect();

        res_set.len() as i32
    }

    pub fn number_of_lines_806(widths: Vec<i32>, s: String) -> Vec<i32> {
        let mut total_lines = 1;
        let mut line_width = 0;

        for &b in s.as_bytes() {
            let index = (b - b'a') as usize;
            if line_width + widths[index] <= 100 {
                line_width += widths[index];
                continue;
            }

            line_width = widths[index];
            total_lines += 1;
        }

        vec![total_lines, line_width]
    }

    pub fn most_common_word_819(paragraph: String, banned: Vec<String>) -> String {
        use std::collections::HashMap;
        paragraph
            .to_ascii_lowercase()
            .split(|c: char| {
                c.is_ascii_whitespace() && matches!(c, '!' | '?' | '\'' | ',' | ';' | '.')
            })
            .filter(|w| !banned.contains(&w.to_string()) && !w.is_empty())
            .fold(HashMap::new(), |mut acc: HashMap<&str, u16>, word: &str| {
                *acc.entry(word).or_insert(1) += 1;
                acc
            })
            .into_iter()
            .max_by_key(|&(_, freq)| freq)
            .unwrap()
            .0
            .to_string()
    }

    pub fn shortest_to_char_821_1(s: String, c: char) -> Vec<i32> {
        let n = s.len();
        let bytes = s.as_bytes();
        let mut res: Vec<i32> = vec![0; n];

        for i in 0..n {
            res[i] = if bytes[i] != c as u8 { n as i32 } else { 0 };
        }

        for i in 1..n {
            res[i] = std::cmp::min(res[i], res[i - 1] + 1);
        }

        for i in (0..=n - 2).rev() {
            res[i] = std::cmp::min(res[i], res[i + 1] + 1);
        }

        res
    }

    pub fn to_goat_latin_824(sentence: String) -> String {
        sentence
            .split_whitespace()
            .into_iter()
            .enumerate()
            .map(|(i, w)| {
                let mut bytes = w.to_string().into_bytes();
                if !matches!(
                    bytes[0],
                    b'A' | b'E' | b'I' | b'O' | b'U' | b'a' | b'e' | b'i' | b'o' | b'u'
                ) {
                    bytes.push(bytes[0]);
                    bytes.remove(0);
                }

                bytes.append(&mut vec![b'm', b'a']);
                bytes.append(&mut vec![b'a'; i + 1]);

                String::from_utf8(bytes).unwrap()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn large_group_positions_830(s: String) -> Vec<Vec<i32>> {
        let n = s.len();
        let bytes: &[u8] = s.as_bytes();

        let mut start = 0;
        let mut res: Vec<Vec<i32>> = vec![];

        for i in 1..n {
            let end: usize;
            if bytes[i] == bytes[i - 1] {
                if i != n - 1 {
                    continue;
                }
                end = i;
            } else {
                end = i - 1;
            }

            if end - start >= 2 {
                res.push(vec![start as i32, end as i32]);
            }

            start = i;
        }

        res
    }

    pub fn backspace_commpare_844(s: String, t: String) -> bool {
        let process_str = |mut s: Vec<u8>| -> String {
            let mut k = 0;

            for i in 0..s.len() {
                if s[i] != b'#' {
                    s[k] = s[i];
                    k += 1;
                    continue;
                }

                if k > 0 {
                    k -= 1;
                }
            }

            unsafe { String::from_utf8_unchecked(s[0..k].to_vec()) }
        };

        process_str(s.into_bytes()) == process_str(t.into_bytes())
    }

    pub fn buddy_strings_859(s: String, goal: String) -> bool {
        use std::collections::HashSet;

        if s.len() != goal.len() {
            return false;
        }

        if s == goal {
            return s.chars().collect::<HashSet<char>>().len() < s.len();
        }

        let mut dif: Vec<usize> = vec![];
        let s_bytes = s.as_bytes();
        let goal_bytes = goal.as_bytes();

        for i in 0..s.len() {
            if s_bytes[i] != goal_bytes[i] {
                dif.push(i);
            }
        }

        dif.len() == 2
            && s_bytes[dif[0]] == goal_bytes[dif[1]]
            && s_bytes[dif[1]] == goal_bytes[dif[0]]
    }

    pub fn uncommon_from_sentences_884(s1: String, s2: String) -> Vec<String> {
        use std::collections::HashMap;
        (s1 + " " + &s2)
            .split_whitespace()
            .fold(HashMap::new(), |mut acc: HashMap<String, u8>, w| {
                acc.entry(w.to_string())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                acc
            })
            .iter()
            .filter_map(|(k, v)| if *v == 1 { Some(k.clone()) } else { None })
            .collect::<Vec<String>>()
    }

    pub fn reverse_only_letters_917_1(s: String) -> String {
        let letter_indexes = s
            .chars()
            .enumerate()
            .filter_map(|(i, b)| {
                if b.is_ascii_alphabetic() {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>();

        let mut bytes = s.into_bytes();

        let n = letter_indexes.len();
        for i in 0..n / 2 {
            bytes.swap(letter_indexes[i], letter_indexes[n - 1 - i]);
        }

        String::from_utf8(bytes).unwrap()
    }

    pub fn reverse_only_letters_917_2(s: String) -> String {
        let is_aplha = |b: u8| -> bool { (b >= 65 && b <= 90) || (b >= 97 && b <= 122) };

        let mut i = 0;
        let mut j = s.len() - 1;
        let mut bytes = s.into_bytes();

        while i < j {
            if !is_aplha(bytes[i]) {
                i += 1;
            } else if !is_aplha(bytes[j]) {
                j -= 1;
            } else {
                bytes.swap(i, j);
                i += 1;
                j -= 1;
            }
        }

        String::from_utf8(bytes).unwrap()
    }

    pub fn is_long_pressed_name_925(name: String, typed: String) -> bool {
        let name_bytes = name.as_bytes();
        let typed_bytes = typed.as_bytes();

        let mut i = 0;

        for j in 0..typed.len() {
            if i < name.len() && name_bytes[i] == typed_bytes[i] {
                i += 1;
            } else if j == 0 || typed_bytes[j] != typed_bytes[j - 1] {
                return false;
            }
        }

        i == name.len()
    }

    pub fn num_unique_emails_929(emails: Vec<String>) -> i32 {
        use std::collections::HashSet;
        emails
            .iter()
            .map(|e| -> String {
                let (local, domain) = e.split_once("@").unwrap();
                let normalized: String = local
                    .chars()
                    .take_while(|c| *c != '+')
                    .filter(|c| *c != '.')
                    .collect();

                normalized + "@" + &domain
            })
            .collect::<HashSet<_>>()
            .len() as i32
    }

    pub fn di_string_match_942(s: String) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut i: usize = 0;
        let mut j: usize = s.len();
        let bytes = s.as_bytes();

        for k in 0..s.len() {
            if bytes[k] == b'D' {
                res.push(j as i32);
                j -= 1;
            } else {
                res.push(i as i32);
                i += 1;
            }
        }

        res.push(i as i32);

        res
    }

    pub fn min_deletion_size_944(strs: Vec<String>) -> i32 {
        let grid: Vec<Vec<u8>> = strs.into_iter().map(|s| s.into_bytes()).collect();
        let m = grid.len();
        let n = grid[1].len();

        let mut res: i32 = 0;

        for j in 0..n {
            for i in 1..m {
                if grid[i - 1][j] > grid[i][j] {
                    res += 1;
                    break;
                }
            }
        }
        res
    }

    pub fn is_alien_sorted_953(words: Vec<String>, order: String) -> bool {
        let mut map_alien = [0u8; 26];
        for i in 0..26 {
            map_alien[(order.as_bytes()[i] - b'a') as usize] = i as u8;
        }

        for i in 1..words.len() {
            let m = words[i - 1].len();
            let n = words[i].len();
            let mut flag = false;

            for j in 0..m.min(n) {
                let c1_idx = (words[i - 1].as_bytes()[j] - b'a') as usize;
                let c2_idx = (words[i].as_bytes()[j] - b'a') as usize;

                if map_alien[c1_idx] < map_alien[c2_idx] {
                    flag = true;
                    break;
                }

                if map_alien[c1_idx] > map_alien[c2_idx] {
                    return false;
                }
            }

            if !flag && m > n {
                return false;
            }
        }

        true
    }

    pub fn common_chars_1002(words: Vec<String>) -> Vec<String> {
        if words.len() == 1 {
            return words[0]
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>();
        }

        let calc_freq = |s: &String| -> Vec<u8> {
            let mut freq = vec![0u8; 26];
            for b in s.as_bytes() {
                freq[(*b - b'a') as usize] += 1;
            }
            freq
        };

        let mut acc_freq = calc_freq(&words[0]);
        for i in 1..words.len() {
            let curr_freq = calc_freq(&words[i]);
            for j in 0..26 {
                if acc_freq[j] != 0 && curr_freq[j] != 0 {
                    acc_freq[j] = acc_freq[j].min(curr_freq[j]);
                }

                if curr_freq[j] == 0 {
                    acc_freq[j] = 0;
                }
            }
        }

        let mut res: Vec<String> = vec![];
        for i in 0..26 {
            if acc_freq[i] > 0 {
                res.extend(vec![
                    String::from_utf8(vec![i as u8 + b'a']).unwrap();
                    acc_freq[i] as usize
                ]);
            }
        }

        res
    }

    pub fn remove_outer_parenthese_1021_1(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut res = String::new();
        let mut sub_str = String::from("(");
        let mut cnt_paren = 1;

        for i in 1..chars.len() {
            if cnt_paren == 0 {
                if sub_str.len() > 2 {
                    res.push_str(&sub_str[1..sub_str.len() - 1]);
                }
                sub_str.clear();
            }

            if chars[i] == '(' {
                cnt_paren += 1;
            } else {
                cnt_paren -= 1;
            }

            sub_str.push(chars[i]);
        }

        if sub_str.len() > 2 {
            res.push_str(&sub_str[1..sub_str.len() - 1]);
        }

        res
    }

    pub fn remove_outer_parenthese_1021_2(s: String) -> String {
        let mut res = String::new();
        let mut opened = 0;

        for c in s.chars() {
            if c == '(' && opened > 0 {
                res.push(c);
            }

            if c == ')' && opened > 1 {
                res.push(c);
            }

            opened += if c == '(' { 1 } else { -1 };
        }

        res
    }

    pub fn remove_duplicates_1047_1(s: String) -> String {
        let bytes: &[u8] = s.as_bytes();
        let mut res: Vec<u8> = vec![];
        let mut stack: Vec<usize> = vec![0];

        for i in 1..bytes.len() {
            if !stack.is_empty() && bytes[stack[stack.len() - 1]] == bytes[i] {
                stack.pop();
            } else {
                stack.push(i);
            }
        }

        for i in 0..stack.len() {
            res.push(bytes[stack[i]]);
        }

        unsafe { String::from_utf8_unchecked(res) }
    }

    pub fn remove_duplicates_1047_2(s: String) -> String {
        let mut s: Vec<u8> = s.into_bytes();
        let mut i = 0;

        for j in 0..s.len() {
            if i == 0 {
                s[i] = s[j];
                i += 1;
            } else if s[i - 1] == s[j] {
                i -= 1;
            } else {
                s[i] = s[j];
                i += 1;
            }
        }

        s.truncate(i);
        unsafe { String::from_utf8_unchecked(s) }
    }

    pub fn gcd_of_strings_1071_1(str1: String, str2: String) -> String {
        let m = str1.len();
        let n = str2.len();

        let mut sub1 = String::new();
        let mut sub2 = String::new();

        let mut res = String::new();
        for i in 0..m.min(n) {
            sub1.push(str1.as_bytes()[i] as char);
            sub2.push(str2.as_bytes()[i] as char);

            if sub1.repeat(m / sub1.len()) == str1
                && sub2.repeat(n / sub2.len()) == str2
                && sub1 == sub2
            {
                res = sub1.clone();
            }
        }

        res
    }

    pub fn gcd_of_strings_1071_2(str1: String, str2: String) -> String {
        let check_divisible = |s1: &str, s2: &str| -> bool {
            if s1.len() % s2.len() == 0 {
                let mut start = 0;
                let offset = s2.len();
                let mut until = start + offset;
                let end = s1.len();

                let mut divisible = true;
                while start < end {
                    if s1[start..until] != s2[..] {
                        divisible = false;
                        break;
                    }
                    start += offset;
                    until += offset;
                }
                return divisible;
            }
            false
        };

        let (shorter, longer): (&str, &str) = if str1.len() < str2.len() {
            (&str1, &str2)
        } else {
            (&str2, &str1)
        };

        if check_divisible(longer, shorter) {
            return String::from(shorter);
        }

        let mut res = String::new();
        let mut divisor = 1;

        while divisor < shorter.len() / 2 {
            if check_divisible(shorter, &shorter[0..divisor]) {
                break;
            }
            divisor += 1;
        }

        if divisor > shorter.len() / 2 {
            return res;
        }

        if check_divisible(longer, &shorter[0..divisor]) {
            res = String::from(&shorter[0..divisor]);
        } else {
            return res;
        }

        let increments = divisor;
        divisor += increments;
        while divisor < shorter.len() {
            if shorter.len() % divisor == 0 && longer.len() % divisor == 0 {
                if check_divisible(longer, &shorter[0..divisor]) {
                    res = String::from(&shorter[0..divisor]);
                } else {
                    return res;
                }
            }
            divisor += increments;
        }

        res
    }

    pub fn gcd_of_strings_1071_3(str1: String, str2: String) -> String {
        let gcd = |mut a: usize, mut b: usize| -> usize {
            while b != 0 {
                let tmp = b;
                b = a % b;
                a = tmp;
            }
            a
        };

        if str1.clone() + &str2 == str2.clone() + &str1 {
            return String::from(&str1[0..gcd(str1.len(), str2.len())]);
        }

        String::new()
    }

    pub fn find_ocurrences_1078(text: String, first: String, second: String) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut res: Vec<String> = vec![];
        for i in 2..words.len() {
            if words[i - 1] == second && words[i - 2] == first {
                res.push(words[i].to_string());
            }
        }
        res
    }

    pub fn defang_ip_addr_1108(address: String) -> String {
        address.replace(".", "[.]")
    }

    pub fn day_of_year_1154(date: String) -> i32 {
        const DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let year = date[..4].parse::<u16>().unwrap();
        let month = date[5..7].parse::<u8>().unwrap();
        let mut day = date[8..].parse::<u16>().unwrap();

        if month == 1 {
            return day as i32;
        }

        for i in 0..(month as usize - 1) {
            day += DAYS_IN_MONTH[i] as u16;
        }

        if ((year % 4 == 0 && year % 100 != 0) || year % 400 == 0) && month > 2 {
            day += 1;
        }

        day as i32
    }

    pub fn count_characters_1160(words: Vec<String>, chars: String) -> i32 {
        let mut chars_freq = [0u8; 26];
        for i in 0..chars.len() {
            chars_freq[(chars.as_bytes()[i] - b'a') as usize] += 1;
        }

        words
            .iter()
            .filter_map(|w| {
                let mut ele_freq = [0u8; 26];
                for i in 0..w.len() {
                    let idx = (w.as_bytes()[i] - b'a') as usize;
                    ele_freq[idx] += 1;
                    if ele_freq[idx] > chars_freq[idx] {
                        return None;
                    }
                }
                Some(w.len() as i32)
            })
            .sum()
    }

    pub fn max_number_of_balloons_1189(text: String) -> i32 {
        let (mut a_cnt, mut b_cnt, mut o_cnt, mut l_cnt, mut n_cnt) = (0, 0, 0, 0, 0);

        for i in 0..text.len() {
            match text.as_bytes()[i] {
                b'a' => a_cnt += 1,
                b'b' => b_cnt += 1,
                b'o' => o_cnt += 1,
                b'l' => l_cnt += 1,
                b'n' => n_cnt += 1,
                _ => (),
            }
        }

        let min_shorter = a_cnt.min(b_cnt.min(n_cnt));

        if min_shorter * 2 < o_cnt.min(l_cnt) {
            return min_shorter;
        }

        o_cnt.min(l_cnt) / 2
    }

    pub fn balanced_string_split_1221(s: String) -> i32 {
        let mut k = 0;
        let mut res = 0;

        for i in 0..s.len() {
            if s.as_bytes()[i] == b'L' {
                k += 1;
            } else {
                k -= 1;
            }

            if k == 0 {
                res += 1;
            }
        }

        res
    }

    pub fn freq_alphabets_1309_1(s: String) -> String {
        let process_str = |s: &str| -> String {
            if s.len() == 0 {
                return String::new();
            }

            let last_char: u8 = s[s.len() - 2..].parse::<u8>().unwrap() - 1 + b'a';
            if s.len() > 2 {
                let mut res: Vec<u8> = vec![];
                for i in 0..s.len() - 2 {
                    res.push(s.as_bytes()[i] - b'1' + b'a');
                }
                res.push(last_char);
                return unsafe { String::from_utf8_unchecked(res) };
            }

            unsafe { String::from_utf8_unchecked(vec![last_char]) }
        };

        let flag = if *(s.as_bytes().last().unwrap()) == b'#' {
            true
        } else {
            false
        };

        let parts: Vec<&str> = s.split('#').collect();
        parts
            .iter()
            .enumerate()
            .map(|(i, sub_str)| -> String {
                if i == parts.len() - 1 && !flag {
                    let mut res: Vec<u8> = vec![];
                    for i in 0..sub_str.len() {
                        res.push(sub_str.as_bytes()[i] - b'1' + b'a');
                    }
                    return unsafe { String::from_utf8_unchecked(res) };
                }

                process_str(sub_str)
            })
            .collect()
    }

    pub fn freq_alphabets_1309_2(mut s: String) -> String {
        for i in (1..=26).rev() {
            let from: String = i.to_string() + if i > 9 { "#" } else { "" };
            let bytes = [b'a' - 1 + i as u8];
            let to = unsafe { std::str::from_utf8_unchecked(&bytes) };
            s = s.replace(&from, to);
        }

        return s;
    }

    pub fn remove_palindrome_string_1332(s: String) -> i32 {
        if s.is_empty() {
            0
        } else if s == s.chars().rev().collect::<String>() {
            1
        } else {
            2
        }
    }

    pub fn days_between_dates_1360(date1: String, date2: String) -> i32 {
        const DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        let count_leap_year = |mut year: i32, month: u8| -> i32 {
            if month <= 2 {
                year -= 1;
            }

            year / 4 - year / 100 + year / 400
        };

        let convert_date = |date: String| -> (i32, u8, i32) {
            let year = date[..4].parse::<i32>().unwrap();
            let month = date[5..7].parse::<u8>().unwrap();
            let day = date[8..].parse::<i32>().unwrap();
            (year, month, day)
        };

        let (y1, m1, d1) = convert_date(date1);
        let (y2, m2, d2) = convert_date(date2);

        let mut days1 = y1 * 365 + d1;
        let mut days2 = y2 * 365 + d2;

        for i in 0..(m1 as usize - 1) {
            days1 += DAYS_IN_MONTH[i] as i32;
        }
        days1 += count_leap_year(y1, m1);

        for i in 0..(m2 as usize - 1) {
            days2 += DAYS_IN_MONTH[i] as i32;
        }
        days2 += count_leap_year(y2, m2);

        (days1 - days2).abs()
    }

    pub fn sort_string_1370_1(s: String) -> String {
        use std::collections::HashMap;

        let mut freq: HashMap<u8, u16> = HashMap::new();
        for b in s.as_bytes() {
            freq.entry(*b).and_modify(|c| *c += 1).or_insert(1);
        }

        let mut keys: Vec<u8> = freq.keys().cloned().collect();
        keys.sort();

        let mut res: Vec<u8> = vec![];
        let mut i: i32 = 0;
        let mut asc = true;
        while freq.len() != 0 {
            if let Some(c) = freq.get_mut(&keys[i as usize]) {
                *c -= 1;
                res.push(keys[i as usize]);
                if *c == 0 {
                    freq.remove(&keys[i as usize]);
                }
            }

            if asc {
                i += 1;
            } else {
                i -= 1;
            }

            if i as usize == keys.len() {
                i -= 1;
                asc = false;
            }

            if i == -1 {
                i += 1;
                asc = true;
            }
        }

        unsafe { String::from_utf8_unchecked(res) }
    }

    pub fn sort_string_1370_2(s: String) -> String {
        let mut freq = [0u16; 26];
        for b in s.as_bytes() {
            freq[(*b - b'a') as usize] += 1;
        }

        let mut res = String::new();
        let add = |cnt: &mut [u16; 26], res: &mut String, asc: bool| {
            for i in 0..26 {
                let j = if asc { i } else { 25 - i };
                if cnt[j] > 0 {
                    cnt[j] -= 1;
                    res.push((j as u8 + b'a') as char)
                }
            }
        };

        while res.len() < s.len() {
            add(&mut freq, &mut res, true);
            add(&mut freq, &mut res, false);
        }

        res
    }

    pub fn generate_the_string_1374(n: i32) -> String {
        if n % 2 != 0 {
            return String::from("a").repeat(n as usize);
        }

        String::from("a").repeat(n as usize - 1) + "b"
    }

    pub fn string_matching_1408_1(words: Vec<String>) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for i in 0..words.len() {
            for j in 0..words.len() {
                if words[j].contains(&words[i]) && i != j {
                    res.push(words[i].clone());
                    break;
                }
            }
        }
        res
    }

    pub fn string_matching_1408_2(mut words: Vec<String>) -> Vec<String> {
        words.sort_unstable_by_key(|w| w.len());
        words
            .iter()
            .enumerate()
            .filter_map(|(i, w)| words[i + 1..].iter().any(|w2| w2.contains(w)).then_some(w))
            .cloned()
            .collect()
    }

    pub fn reformat_1417(s: String) -> String {
        use std::cmp::min;

        let mut letters: Vec<u8> = vec![];
        let mut digits: Vec<u8> = vec![];

        for i in 0..s.len() {
            let b = s.as_bytes()[i];
            if b >= b'a' && b <= b'z' {
                letters.push(b);
            } else {
                digits.push(b);
            }
        }

        if letters.len().abs_diff(digits.len()) > 1 {
            return String::new();
        }

        let mut bytes: Vec<u8> = vec![];
        for i in 0..min(letters.len(), digits.len()) {
            bytes.push(letters[i]);
            bytes.push(digits[i]);
        }

        let res: String = unsafe { String::from_utf8_unchecked(bytes) };
        if letters.len() == digits.len() {
            res
        } else if letters.len() > digits.len() {
            res + &(*letters.last().unwrap() as char).to_string()
        } else {
            (*digits.last().unwrap() as char).to_string() + &res
        }
    }

    pub fn max_score_1422(s: String) -> i32 {
        let cnt_one = s.as_bytes().iter().filter(|b| **b == b'1').count();

        let mut res = 0;
        let mut cnt_zero_left = 0;
        for i in 0..s.len() - 1 {
            if s.as_bytes()[i] == b'0' {
                cnt_zero_left += 1;
            }
            let cnt_one_right = cnt_one - (i + 1 - cnt_zero_left);
            res = res.max(cnt_zero_left + cnt_one_right);
        }

        res as i32
    }

    pub fn dest_city_1436_1(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashMap;
        let mut des = paths[0][1].clone();
        let map: HashMap<String, String> = paths
            .iter()
            .map(|path| (path[0].clone(), path[1].clone()))
            .collect();

        while let Some(to) = map.get(&des) {
            des = to.clone();
        }

        des
    }

    pub fn dest_city_1436_2(paths: Vec<Vec<String>>) -> String {
        use std::collections::HashSet;
        let set: HashSet<String> = paths.iter().map(|path| path[0].clone()).collect();
        for path in paths.iter() {
            if !set.contains(&path[1]) {
                return path[1].clone();
            }
        }
        String::new()
    }

    pub fn max_power_1446(s: String) -> i32 {
        if s.len() == 1 {
            return 1;
        }

        let mut res = 1;
        let mut cnt = 1;
        let mut prev = s.as_bytes()[0];

        for i in 1..s.len() {
            let curr = s.as_bytes()[i];

            if curr == prev {
                cnt += 1;
            } else {
                cnt = 1;
                prev = curr;
            }
            res = res.max(cnt)
        }

        res
    }

    pub fn is_prefix_of_word_1455(sentence: String, search_word: String) -> i32 {
        for (i, word) in sentence.split_whitespace().enumerate() {
            if word.len() >= search_word.len() && word[0..search_word.len()] == search_word {
                return i as i32 + 1;
            }
        }
        -1
    }

    pub fn is_path_crossing_1496(path: String) -> bool {
        use std::collections::HashSet;

        let mut visited_point: HashSet<(i16, i16)> = HashSet::new();
        let mut curr: (i16, i16) = (0, 0);
        visited_point.insert(curr);
        for c in path.chars() {
            match c {
                'N' => curr.1 += 1,
                'S' => curr.1 -= 1,
                'E' => curr.0 += 1,
                'W' => curr.0 -= 1,
                _ => (),
            }
            if !visited_point.insert(curr) {
                return true;
            }
        }

        false
    }

    pub fn reformat_date_1507(date: String) -> String {
        use std::collections::HashMap;
        let month: HashMap<&str, &str> = HashMap::from([
            ("Jan", "01"),
            ("Feb", "02"),
            ("Mar", "03"),
            ("Apr", "04"),
            ("May", "05"),
            ("Jun", "06"),
            ("Jul", "07"),
            ("Aug", "08"),
            ("Sep", "09"),
            ("Oct", "10"),
            ("Nov", "11"),
            ("Dec", "12"),
        ]);

        let dmy: Vec<&str> = date.split_whitespace().collect();
        let mut day = (&dmy[0][0..dmy[0].len() - 2]).to_string();
        if day.len() == 1 {
            day = String::from("0") + &day;
        }
        format!("{}-{}-{}", dmy[2], month.get(&dmy[1]).unwrap(), day)
    }

    pub fn restore_string_1528(s: String, indices: Vec<i32>) -> String {
        let mut res = vec![0u8; indices.len()];
        for i in 0..indices.len() {
            res[indices[i] as usize] = s.as_bytes()[i];
        }
        unsafe { String::from_utf8_unchecked(res) }
    }

    pub fn make_good_1544(s: String) -> String {
        let mut res: Vec<u8> = vec![];
        for i in 0..s.len() {
            if res.len() != 0
                && *res.last().unwrap() != s.as_bytes()[i]
                && (*res.last().unwrap()).eq_ignore_ascii_case(&s.as_bytes()[i])
            {
                res.pop();
            } else {
                res.push(s.as_bytes()[i]);
            }
        }

        unsafe { String::from_utf8_unchecked(res) }
    }

    pub fn thousand_separator_1556(n: i32) -> String {
        let s = n.to_string();
        let mut res = String::new();
        for i in 0..s.len() {
            if i > 0 && (s.len() - i) % 3 == 0 {
                res.push('.');
            }

            res.push(s.as_bytes()[i] as char);
        }

        res
    }

    pub fn modify_string_1576(mut s: String) -> String {
        for i in 0..s.len() {
            if s.as_bytes()[i] == b'?' {
                for c in b'a'..=b'z' {
                    if (i == 0 || s.as_bytes()[i - 1] != c)
                        && (i == s.len() - 1 || s.as_bytes()[i + 1] != c)
                    {
                        unsafe { s.as_bytes_mut()[i] = c };
                        break;
                    }
                }
            }
        }
        s
    }

    pub fn reorder_spaces_1592(text: String) -> String {
        let words: Vec<&str> = text.split_whitespace().collect();
        let letters_len: usize = words.iter().map(|&s| s.len()).sum();
        let spaces_len = text.len() - letters_len;

        if words.len() == 1 {
            return words[0].to_owned() + &(" ".repeat(spaces_len));
        }

        words.join(&(" ".repeat(spaces_len / (words.len() - 1))))
            + &(" ".repeat(spaces_len % (words.len() - 1)))
    }

    pub fn min_operations_1598(logs: Vec<String>) -> i32 {
        let mut res = 0;
        for log in logs.iter() {
            match log.as_str() {
                "../" => {
                    if res > 0 {
                        res -= 1
                    }
                }
                "./" => (),
                _ => res += 1,
            }
        }

        res
    }

    pub fn max_depth_1614(s: String) -> i32 {
        let mut stack = 0;
        let mut res = 0;
        for i in 0..s.len() {
            match s.as_bytes()[i] {
                b'(' => {
                    stack += 1;
                    res = res.max(stack);
                }
                b')' => stack -= 1,
                _ => (),
            }
        }

        res
    }

    pub fn max_length_between_equal_characters_1624(s: String) -> i32 {
        use std::collections::HashMap;
        let mut idx_map: HashMap<u8, (u16, i16)> = HashMap::new();
        for i in 0..s.len() {
            idx_map
                .entry(s.as_bytes()[i])
                .and_modify(|(idx, len_sub)| *len_sub = i as i16 - *idx as i16 - 1)
                .or_insert((i as u16, -1));
        }

        idx_map.values().map(|(_, len_sub)| *len_sub).max().unwrap() as i32
    }

    pub fn slowest_key_1629(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut max_dura = release_times[0];
        let mut res: u8 = keys_pressed.as_bytes()[0];
        for i in 1..release_times.len() {
            let curr_dura = release_times[i] - release_times[i - 1];

            if curr_dura == max_dura {
                res = res.max(keys_pressed.as_bytes()[i]);
                continue;
            }

            if curr_dura > max_dura {
                max_dura = curr_dura;
                res = keys_pressed.as_bytes()[i];
            }
        }

        res as char
    }

    pub fn array_strings_are_equal_1662(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1.join("") == word2.join("")
    }

    pub fn max_repeating_1668(sequence: String, word: String) -> i32 {
        let mut res = 0;
        let mut tmp = word.clone();
        while sequence.find(&tmp).is_some() {
            res += 1;
            tmp += &word;
        }
        res
    }

    pub fn interpret_1678(command: String) -> String {
        let mut res = String::new();
        let mut i: usize = 0;
        while i < command.len() {
            if command.as_bytes()[i] == b'G' {
                res.push('G');
                i += 1;
                continue;
            }

            if command.as_bytes()[i + 1] == b')' {
                res.push('o');
                i += 2;
                continue;
            }

            res.push_str("al");
            i += 4;
        }

        res
    }

    pub fn count_consistent_strings_1684(allowed: String, words: Vec<String>) -> i32 {
        let mut allowed_chars = [false; 26];
        for i in 0..allowed.len() {
            allowed_chars[(allowed.as_bytes()[i] - b'a') as usize] = true;
        }

        words
            .iter()
            .filter(|&w| {
                w.as_bytes()
                    .iter()
                    .all(|&c| allowed_chars[(c - b'a') as usize])
            })
            .count() as i32
    }

    pub fn reformat_number_1694(number: String) -> String {
        let mut res = String::new();
        let mut len_sub: u8 = 0;
        let mut buffer = String::new();
        for c in number.chars() {
            if c == ' ' || c == '-' {
                continue;
            }

            len_sub += 1;
            buffer.push(c);
            if len_sub == 3 {
                res += &format!("{}-", buffer);
                len_sub = 0;
                buffer.clear();
            }
        }

        if buffer.len() == 0 {
            res.pop().unwrap();
        }

        if buffer.len() == 1 {
            let n = res.len();
            unsafe { res.as_bytes_mut().swap(n - 2, n - 1) };
        }

        res + &buffer
    }

    pub fn halves_are_alike_1704(s: String) -> bool {
        let count_vowels = |s: &str| -> u16 {
            s.chars()
                .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U'))
                .count() as u16
        };

        count_vowels(&s[..s.len() / 2]) == count_vowels(&s[s.len() / 2..])
    }

    pub fn maximum_time_1736(mut time: String) -> String {
        unsafe {
            let s = time.as_bytes_mut();
            if s[0] == b'?' {
                s[0] = if s[1] == b'?' || s[1] <= b'3' {
                    b'2'
                } else {
                    b'1'
                };
            }

            if s[1] == b'?' {
                s[1] = if s[0] == b'2' { b'3' } else { b'9' };
            }

            if s[3] == b'?' {
                s[3] = b'5';
            }

            if s[4] == b'?' {
                s[4] = b'9';
            }

            std::str::from_utf8_unchecked(s).to_string()
        }
    }

    pub fn min_operations_1758(s: String) -> i32 {
        let mut res = 0;
        for i in 0..s.len() {
            if (s.as_bytes()[i] - b'0') as usize != i % 2 {
                res += 1;
            }
        }
        res.min(s.len() as i32 - res)
    }

    pub fn longest_nice_substring_1763(s: String) -> String {
        use std::collections::HashSet;

        fn find_longest(s: &str) -> String {
            if s.len() < 2 {
                return String::new();
            }

            let set: HashSet<char> = s.chars().collect();

            for (i, c) in s.chars().enumerate() {
                if set.contains(&c.to_ascii_uppercase()) && set.contains(&c.to_ascii_lowercase()) {
                    continue;
                }

                let sub1 = find_longest(&s[0..i]);
                let sub2 = find_longest(&s[i + 1..]);
                return if sub1.len() >= sub2.len() { sub1 } else { sub2 };
            }

            s.to_string()
        }

        find_longest(s.as_str())
    }

    pub fn merge_alternately_1768(word1: String, word2: String) -> String {
        let mut i = 0;
        let mut j = 0;
        let mut res: Vec<u8> = vec![];

        while i < word1.len() || j < word2.len() {
            if i < word1.len() {
                res.push(word1.as_bytes()[i]);
                i += 1;
            }

            if j < word2.len() {
                res.push(word2.as_bytes()[j]);
                j += 1;
            }
        }

        unsafe { String::from_utf8_unchecked(res) }
    }

    pub fn count_matches_1773(
        items: Vec<Vec<String>>,
        rule_key: String,
        rule_value: String,
    ) -> i32 {
        let idx: usize = match rule_key.as_str() {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => panic!(),
        };

        items.iter().filter(|ele| ele[idx] == rule_value).count() as i32
    }

    pub fn check_one_segment_1784(s: String) -> bool {
        !s.contains("01")
    }

    pub fn are_almost_equal_1790(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }

        let mut diff: Vec<usize> = vec![];
        for i in 0..s1.len() {
            if s1.as_bytes()[i] != s2.as_bytes()[i] {
                diff.push(i);
            }

            if diff.len() > 2 {
                return false;
            }
        }

        if diff.len() != 2 {
            return false;
        }

        s1.as_bytes()[diff[0]] == s2.as_bytes()[diff[1]]
            && s1.as_bytes()[diff[1]] == s2.as_bytes()[diff[0]]
    }

    pub fn second_highest_1796(s: String) -> i32 {
        let mut first = -1;
        let mut sec = -1;
        for i in 0..s.len() {
            if s.as_bytes()[i].is_ascii_digit() {
                let d = (s.as_bytes()[i] - b'0') as i32;
                if first < d {
                    (sec, first) = (first, d);
                } else if sec < d && d < first {
                    sec = d;
                }
            }
        }

        sec
    }

    pub fn num_different_integers_1805(word: String) -> i32 {
        use std::collections::HashSet;
        word.split(char::is_alphabetic)
            .filter_map(|s| {
                if s != "" {
                    Some(s.trim_start_matches('0'))
                } else {
                    None
                }
            })
            .collect::<HashSet<&str>>()
            .len() as i32
    }

    pub fn square_is_white_1812(coordinates: String) -> bool {
        ((coordinates.as_bytes()[0] - b'a') + (coordinates.as_bytes()[1] - b'1')) % 2 == 1
    }

    pub fn truncate_sentence_1816(s: String, k: i32) -> String {
        let mut count_spaces = 0;
        for i in 0..s.len() {
            if s.as_bytes()[i].is_ascii_whitespace() {
                count_spaces += 1;
                if count_spaces == k {
                    return (&s[..i]).to_string();
                }
            }
        }

        return s;
    }

    pub fn check_if_pangram_1832(sentence: String) -> bool {
        let mut freq = [0u16; 26];
        for i in 0..sentence.len() {
            freq[(sentence.as_bytes()[i] - b'a') as usize] += 1;
        }

        freq.iter().all(|cnt| *cnt > 0)
    }

    pub fn replace_digits_1844(mut s: String) -> String {
        for i in (1..s.len()).step_by(2) {
            unsafe {
                s.as_bytes_mut()[i] = s.as_bytes()[i - 1] + s.as_bytes()[i] - b'0';
            }
        }
        return s;
    }

    pub fn sort_sentence_1859(s: String) -> String {
        let mut words = s.split_ascii_whitespace().collect::<Vec<&str>>();

        words.sort_unstable_by_key(|&s| s.as_bytes().last().unwrap());

        words
            .into_iter()
            .map(|s| &s[..s.len() - 1])
            .collect::<Vec<&str>>()
            .join(" ")
    }

    pub fn check_zero_ones_1869(s: String) -> bool {
        let mut lcs0 = 0;
        let mut lcs1 = 0;

        let mut cnt0 = 0;
        let mut cnt1 = 0;

        for c in s.chars() {
            if c == '0' {
                cnt1 = 0;
                cnt0 += 1;
            } else {
                cnt0 = 0;
                cnt1 += 1;
            }
            lcs1 = lcs1.max(cnt1);
            lcs0 = lcs0.max(cnt0);
        }

        lcs1 > lcs0
    }

    pub fn count_good_substrings_1876(s: String) -> i32 {
        use std::collections::HashSet;
        if s.len() < 3 {
            return 0;
        }

        let is_good_str = |s: &str| -> bool {
            s.as_bytes().iter().cloned().collect::<HashSet<u8>>().len() == s.len()
        };

        let mut res = 0;
        for i in 0..s.len() - 2 {
            if is_good_str(&s[i..i + 3]) {
                res += 1;
            }
        }
        res
    }

    pub fn is_sum_equal_1880(first_word: String, second_word: String, target_word: String) -> bool {
        let convert = |s: &str| -> u32 {
            let mut res: u32 = 0;
            for i in 0..s.len() {
                res = res * 10 + (s.as_bytes()[i] - b'a') as u32;
            }
            res
        };

        convert(&first_word) + convert(&second_word) == convert(&target_word)
    }

    pub fn make_equal_1897(words: Vec<String>) -> bool {
        let mut freq = [0; 26];
        words.iter().for_each(|s| {
            for i in 0..s.len() {
                freq[(s.as_bytes()[i] - b'a') as usize] += 1;
            }
        });

        freq.iter().all(|cnt| *cnt % words.len() == 0)
    }

    pub fn largest_odd_numbers_1903(num: String) -> String {
        for i in (0..num.len()).rev() {
            if (num.as_bytes()[i] - b'0') % 2 == 1 {
                return (&num[..i + 1]).to_string();
            }
        }
        String::new()
    }

    pub fn can_be_typed_words_1935(text: String, broken_letters: String) -> i32 {
        let mut broken = [false; 26];
        for i in 0..broken_letters.len() {
            broken[(broken_letters.as_bytes()[i] - b'a') as usize] = true;
        }

        let mut res = 0;
        let mut cnt = 0;

        for i in 0..text.len() {
            if text.as_bytes()[i] == b' ' {
                if cnt == 0 {
                    res += 1;
                }
                cnt = 0;
                continue;
            }
            if broken[(text.as_bytes()[i] - b'a') as usize] {
                cnt += 1;
            }
        }

        if cnt == 0 {
            return res + 1;
        }

        res
    }

    pub fn are_occurrences_equal_1941(s: String) -> bool {
        use std::collections::HashSet;
        let mut freq = [0u16; 26];
        for i in 0..s.len() {
            freq[(s.as_bytes()[i] - b'a') as usize] += 1;
        }

        freq.iter()
            .filter(|c| **c > 0)
            .collect::<HashSet<_>>()
            .len()
            == 2
    }

    pub fn get_lucky_1945(s: String, mut k: i32) -> i32 {
        let mut converted_str = String::new();
        for i in 0..s.len() {
            converted_str.push_str(&(s.as_bytes()[i] - b'a' + 1).to_string())
        }

        let sum_digits = |s: String| -> String {
            s.as_bytes()
                .iter()
                .fold(0u32, |acc, &b| acc + (b - b'0') as u32)
                .to_string()
        };

        while k > 0 {
            converted_str = sum_digits(converted_str);
            k -= 1;
        }

        converted_str.parse::<i32>().unwrap()
    }

    pub fn make_fancy_string_1957(s: String) -> String {
        let mut s = s.into_bytes();
        let mut i = 0;
        let mut prev = (0, true);

        for j in 0..s.len() {
            let c = s[j];
            let b = c != prev.0;
            s[i] = c;
            i += (b || prev.1) as usize;
            prev = (c, b);
        }

        s.truncate(i);
        unsafe { String::from_utf8_unchecked(s) }
    }

    pub fn is_prefix_string_1961(s: String, words: Vec<String>) -> bool {
        let mut buf = String::new();
        for w in words {
            buf += &w;
            if s == buf {
                return true;
            }
        }
        false
    }

    pub fn num_of_strings_1967(patterns: Vec<String>, word: String) -> i32 {
        patterns
            .into_iter()
            .filter(|s| {
                if s.len() > word.len() {
                    return false;
                }

                if *s == word {
                    return true;
                }

                for i in 0..word.len() - s.len() + 1 {
                    if *s == word[i..i + s.len()] {
                        return true;
                    }
                }

                false
            })
            .count() as i32
    }

    pub fn min_time_to_type_1974(word: String) -> i32 {
        let mut res = (word.as_bytes()[0] - b'a').min(26 - word.as_bytes()[0] + b'a') as u16;
        for i in 1..word.len() {
            let diff = word.as_bytes()[i].abs_diff(word.as_bytes()[i - 1]);
            res += diff.min(26 - diff) as u16;
        }

        res as i32 + word.len() as i32
    }

    pub fn reverse_prefix_2000(word: String, ch: char) -> String {
        for (i, c) in word.chars().enumerate() {
            if c == ch {
                let mut res = word.clone().into_bytes();
                for j in 0..(i + 1) / 2 {
                    res.swap(j, i - j);
                }
                return unsafe { String::from_utf8_unchecked(res) };
            }
        }
        word
    }

    pub fn finc_value_after_operations_2011(operations: Vec<String>) -> i32 {
        operations.iter().fold(0i32, |acc, item| {
            acc + match item.as_str() {
                "X++" | "++X" => 1,
                "X--" | "--X" => -1,
                _ => 0,
            }
        })
    }

    pub fn minimum_moves_2027(s: String) -> i32 {
        let mut res = 0;
        let mut i = 0;
        while i < s.len() {
            if s.as_bytes()[i] == b'X' {
                res += 1;
                i += 2;
            }
            i += 1;
        }
        res
    }

    pub fn are_numbers_ascending_2042(s: String) -> bool {
        let numbers = s
            .split_whitespace()
            .filter_map(|token| {
                if token.as_bytes()[0].is_ascii_digit() {
                    Some(token.parse::<u8>().unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<u8>>();

        for i in 1..numbers.len() {
            if numbers[i] <= numbers[i - 1] {
                return false;
            }
        }

        true
    }

    pub fn count_valid_words_2047(sentence: String) -> i32 {
        let rule1 = |w: &str| w.bytes().all(|c| !c.is_ascii_digit());
        let rule2 = |w: &str| {
            if let Some(pos) = w.find('-') {
                0 < pos
                    && pos < w.len() - 1
                    && w.as_bytes()[pos + 1].is_ascii_lowercase()
                    && w[pos + 1..].find('-').is_none()
            } else {
                true
            }
        };
        let rule3 = |w: &str| {
            w[..w.len() - 1]
                .bytes()
                .all(|c| c != b'!' && c != b'.' && c != b',')
        };

        sentence
            .split(' ')
            .filter(|w| !w.is_empty() && rule1(w) && rule2(w) && rule3(w))
            .count() as i32
    }

    pub fn ktd_distinct_2053(arr: Vec<String>, k: i32) -> String {
        use std::collections::HashMap;

        let mut occurrencies: HashMap<&String, usize> = HashMap::new();

        for s in arr.iter() {
            *occurrencies.entry(s).or_default() += 1;
        }

        arr.iter()
            .filter(|s| occurrencies[*s] == 1)
            .nth((k - 1) as usize)
            .unwrap_or(&"".to_string())
            .to_string()
    }

    pub fn count_vowel_substrings_2062_1(word: String) -> i32 {
        // bad code: 70ms :(((
        use std::collections::HashSet;
        let is_vowel_substr = |s: &str| -> bool { s.chars().collect::<HashSet<char>>().len() == 5 };

        let arr: Vec<&str> = word
            .split(|c| !matches!(c, 'u' | 'a' | 'e' | 'o' | 'i'))
            .collect();

        let mut res = 0;
        for s in arr.iter() {
            if s.len() < 5 {
                continue;
            }

            for i in 0..s.len() - 4 {
                for j in i + 5..s.len() + 1 {
                    if is_vowel_substr(&s[i..j]) {
                        res += 1;
                    }
                }
            }
        }

        res
    }

    pub fn count_vowel_substrings_2062_2(word: String) -> i32 {
        let mut res = 0;
        let word = word.as_bytes();

        'outer: for m in 0..word.len() {
            let mut a = 0;
            let mut e = 0;
            let mut i = 0;
            let mut o = 0;
            let mut u = 0;
            for n in m..word.len() {
                match word[n] as char {
                    'a' => a += 1,
                    'e' => e += 1,
                    'i' => i += 1,
                    'o' => o += 1,
                    'u' => u += 1,
                    _ => continue 'outer,
                }
                if a > 0 && e > 0 && i > 0 && o > 0 && u > 0 {
                    res += 1;
                }
            }
        }
        res
    }

    pub fn check_almost_equivalent_2068(word1: String, word2: String) -> bool {
        let calc_freq = |s: &str| -> [u8; 26] {
            s.as_bytes().iter().fold([0u8; 26], |mut acc, b| {
                acc[(*b - b'a') as usize] += 1;
                acc
            })
        };

        let f1 = calc_freq(&word1);
        let f2 = calc_freq(&word2);

        for i in 0..26 {
            if f1[i].abs_diff(f2[i]) > 3 {
                return false;
            }
        }

        true
    }

    pub fn count_words_2085_1(words1: Vec<String>, words2: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let cal_freq = |arr: Vec<String>| -> HashMap<String, u16> {
            arr.iter().fold(HashMap::new(), |mut acc, s| {
                acc.entry(s.to_string())
                    .and_modify(|c| *c += 1)
                    .or_insert(1);
                acc
            })
        };

        let f1 = cal_freq(words1);
        let f2 = cal_freq(words2);

        f1.iter()
            .filter(|(k, v1)| **v1 == 1 && f2.get(*k).is_some_and(|v2| *v2 == 1))
            .count() as i32
    }

    pub fn count_words_2085_2(words1: Vec<String>, words2: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let mut freq: HashMap<String, i16> = HashMap::new();

        for s in words1.iter() {
            freq.entry(s.to_string())
                .and_modify(|c| *c += 1)
                .or_insert(1);
        }

        for s in words2.iter() {
            freq.entry(s.to_string()).or_insert(0);

            if *freq.get(s).unwrap() < 2 {
                *freq.get_mut(s).unwrap() -= 1;
            }
        }

        freq.values().filter(|v| **v == 0).count() as i32
    }

    pub fn count_points_2103(rings: String) -> i32 {
        let mut rods = [[0u8; 3]; 10];
        for i in (1..rings.len()).step_by(2) {
            let idx_rod = (rings.as_bytes()[i] - b'0') as usize;
            match rings.as_bytes()[i - 1] {
                b'R' => rods[idx_rod][0] += 1,
                b'G' => rods[idx_rod][1] += 1,
                b'B' => rods[idx_rod][2] += 1,
                _ => (),
            }
        }

        rods.iter().filter(|r| r.iter().all(|c| *c > 0)).count() as i32
    }

    pub fn first_palindrome_2108(words: Vec<String>) -> String {
        // 2025 Happy New Year :))
        words
            .iter()
            .find(|&s| {
                if s.len() == 0 {
                    return true;
                }

                for i in 0..s.len() / 2 {
                    if s.as_bytes()[i] != s.as_bytes()[s.len() - 1 - i] {
                        return false;
                    }
                }

                true
            })
            .unwrap_or(&String::new())
            .to_string()
    }

    pub fn most_words_found_2114(sentences: Vec<String>) -> i32 {
        sentences
            .iter()
            .map(|w| w.split_ascii_whitespace().collect::<Vec<&str>>().len())
            .max()
            .unwrap() as i32
    }

    pub fn check_string_2124(s: String) -> bool {
        !s.find("ba").is_some()
    }

    pub fn capitalize_title_2129(title: String) -> String {
        title
            .split_ascii_whitespace()
            .map(|w| {
                if w.len() <= 2 {
                    return w.to_lowercase();
                }

                w[0..1].to_ascii_uppercase() + &w[1..].to_ascii_lowercase()
            })
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn divide_string_2138(mut s: String, k: i32, fill: char) -> Vec<String> {
        if s.len() % k as usize != 0 {
            s.push_str(&fill.to_string().repeat(k as usize - s.len() % k as usize))
        };

        s.into_bytes()
            .chunks(k as usize)
            .map(|bytes| unsafe { std::str::from_utf8_unchecked(bytes).to_string() })
            .collect::<Vec<String>>()
    }

    pub fn prefix_count_2185(words: Vec<String>, pref: String) -> i32 {
        words
            .iter()
            .filter(|w| {
                if w.len() < pref.len() {
                    return false;
                }

                w[0..pref.len()] == pref
            })
            .count() as i32
    }

    pub fn cell_in_range_2194(s: String) -> Vec<String> {
        let mut res: Vec<String> = vec![];
        for i in s.as_bytes()[0]..=s.as_bytes()[3] {
            for j in s.as_bytes()[1]..=s.as_bytes()[4] {
                res.push(format!("{}{}", i as char, j as char));
            }
        }
        res
    }

    pub fn convert_time_2224(current: String, correct: String) -> i32 {
        let calc_time = |time: String| -> u16 {
            &time[..2].parse::<u16>().unwrap() * 60 + &time[3..].parse::<u16>().unwrap()
        };

        let curr = calc_time(current);
        let corr = calc_time(correct);

        let mut diff = if corr >= curr {
            corr - curr
        } else {
            24 * 60 - curr + corr
        };

        let mut res = 0;
        let div = [60, 15, 5, 1];
        for d in div.iter() {
            res += diff / d;
            diff = diff % d;
        }

        res as i32
    }

    pub fn digit_sum_2243(s: String, k: i32) -> String {
        fn recur(s: String, k: usize) -> String {
            if s.len() <= k {
                return s;
            }

            return recur(
                s.into_bytes()
                    .chunks(k)
                    .map(|c| {
                        c.iter()
                            .map(|b| (*b - b'0') as u16)
                            .sum::<u16>()
                            .to_string()
                    })
                    .collect::<Vec<String>>()
                    .join(""),
                k,
            );
        }

        recur(s, k as usize)
    }

    pub fn count_prefixes_2255(words: Vec<String>, s: String) -> i32 {
        words
            .iter()
            .filter(|w| {
                if w.len() > s.len() {
                    return false;
                }

                *w == &s[..w.len()]
            })
            .count() as i32
    }

    pub fn remove_digit_2259(number: String, digit: char) -> String {
        let occur = (0..number.len())
            .into_iter()
            .filter(|i| number.as_bytes()[*i] as char == digit)
            .collect::<Vec<usize>>();

        for idx in occur.iter() {
            if *idx < number.len() - 1 {
                if number.as_bytes()[*idx] < number.as_bytes()[*idx + 1] {
                    if *idx == 0 {
                        return number[1..].to_string();
                    }

                    return number[..*idx].to_string() + &number[*idx + 1..];
                }
            }
        }

        let lastest_idx = *occur.last().unwrap();

        if lastest_idx == number.len() - 1 {
            return number[..number.len() - 1].to_string();
        }

        number[..lastest_idx].to_string() + &number[lastest_idx + 1..]
    }

    pub fn largest_good_integer_2264(num: String) -> String {
        let mut res = String::new();
        for i in 0..num.len() - 2 {
            let k = &num[i..i + 3];
            if k.as_bytes()[0] == k.as_bytes()[1] && k.as_bytes()[0] == k.as_bytes()[2] {
                if (res.len() > 0 && res.as_bytes()[0] < k.as_bytes()[0]) || res.len() == 0 {
                    res = k.to_string();
                }
            }
        }
        res
    }

    pub fn divisor_substrings_2269(num: i32, mut k: i32) -> i32 {
        let mut res = 0;
        let mut curr = 0;
        let mut pow = 1;

        let mut n = num;
        while n > 0 {
            curr += (n % 10) * pow;
            k -= 1;
            if k > 0 {
                pow *= 10;
            } else {
                res += if curr != 0 && num % curr == 0 { 1 } else { 0 };
                curr /= 10;
            }
            n /= 10;
        }

        res
    }

    pub fn remove_anagrams_2273(words: Vec<String>) -> Vec<String> {
        let mut prev: Vec<u8> = vec![];
        let mut res: Vec<String> = vec![];

        for i in 0..words.len() {
            let mut curr = words[i].as_bytes().to_vec();
            curr.sort_unstable();

            if curr != prev {
                res.push(words[i].clone());
                prev = curr;
            }
        }

        res
    }

    pub fn percentage_letter_2278(s: String, letter: char) -> i32 {
        let freq = s.chars().fold(0usize, |mut cnt, c| {
            if c == letter {
                cnt += 1
            }
            cnt
        });
        (freq * 100 / s.len()) as i32
    }

    pub fn digit_count_2283(num: String) -> bool {
        let mut freq = [0u8; 10];
        for i in 0..num.len() {
            freq[(num.as_bytes()[i] - b'0') as usize] += 1;
        }
        (0..num.len()).all(|i| freq[i] == num.as_bytes()[i] - b'0')
    }

    pub fn rearrange_characters_2287(s: String, target: String) -> i32 {
        let calc_freq = |s: &str| -> [u8; 26] {
            s.as_bytes().iter().fold([0u8; 26], |mut freq, b| {
                freq[(b - b'a') as usize] += 1;
                freq
            })
        };

        let f1 = calc_freq(&s);
        let f2 = calc_freq(&target);

        f1.iter()
            .zip(f2)
            .filter_map(|(&c1, c2)| {
                if c2 == 0 {
                    return None;
                }
                Some(c1 / c2)
            })
            .min()
            .unwrap() as i32
    }

    pub fn strong_password_checker_ii_2299(password: String) -> bool {
        if password.len() < 8 {
            return false;
        }

        let spec_chars = "!@#$%^&*()-+".to_string();
        let mut c_ll = 0;
        let mut c_ul = 0;
        let mut c_d = 0;
        let mut c_sp = 0;
        let mut prev = b' ';

        for i in 0..password.len() {
            let c = password.as_bytes()[i];

            if c == prev {
                return false;
            }

            match c {
                c if c.is_ascii_digit() => c_d += 1,
                c if c.is_ascii_uppercase() => c_ul += 1,
                c if c.is_ascii_lowercase() => c_ll += 1,
                c if spec_chars.contains(c as char) => c_sp += 1,
                _ => (),
            }

            prev = c;
        }

        c_ll > 0 && c_ul > 0 && c_d > 0 && c_sp > 0
    }

    pub fn greatest_letter_2309(s: String) -> String {
        let mut freq_upper = [0u16; 26];
        let mut freq_lower = [0u16; 26];

        let mut res: u8 = 0;
        s.as_bytes().iter().for_each(|b| {
            if b.is_ascii_uppercase() {
                freq_lower[(b - b'A') as usize] += 1;
            } else {
                freq_upper[(b - b'a') as usize] += 1;
            }

            if freq_upper[(b.to_ascii_uppercase() - b'A') as usize] > 0
                && freq_lower[(b.to_ascii_lowercase() - b'a') as usize] > 0
            {
                res = res.max(b.to_ascii_uppercase());
            }
        });

        if res == 0 {
            return String::new();
        }

        (res as char).to_string()
    }

    pub fn count_asterisks_2315(s: String) -> i32 {
        let mut res = 0;
        let mut flag = false;
        s.chars().for_each(|c| {
            if !flag && c == '*' {
                res += 1;
            }

            if c == '|' {
                flag = !flag
            }
        });

        res
    }

    pub fn decode_message_2325(key: String, message: String) -> String {
        let mut map_code = [-1i8; 26];
        let mut i = 0;
        key.as_bytes().iter().for_each(|b| {
            if i > 25 {
                return;
            }

            if *b != b' ' && map_code[(*b - b'a') as usize] == -1 {
                map_code[(*b - b'a') as usize] = i;
                i += 1;
            }
        });

        message
            .as_bytes()
            .iter()
            .map(|b| {
                if *b == b' ' {
                    return ' ';
                }

                (map_code[(b - b'a') as usize] as u8 + b'a') as char
            })
            .collect::<String>()
    }

    pub fn repeated_character_2351(s: String) -> char {
        let mut cnt = [0u8; 26];
        for c in s.chars() {
            let i = (c as u8 - b'a') as usize;
            cnt[i] += 1;
            if cnt[i] == 2 {
                return c;
            }
        }
        return ' ';
    }

    pub fn minimum_recolors_2379(blocks: String, k: i32) -> i32 {
        let mut cw = 0u8;
        let mut res = u8::MAX;
        let mut prev = 0usize;
        for i in 0..blocks.len() {
            if blocks.as_bytes()[i] == b'W' {
                cw += 1;
            }

            if i - prev + 1 == k as usize {
                res = res.min(cw);
                if blocks.as_bytes()[prev] == b'W' {
                    cw -= 1;
                }
                prev += 1;
            }
        }

        res as i32
    }

    pub fn check_distances_2399(s: String, distance: Vec<i32>) -> bool {
        let mut d = [0u8; 26];
        for i in 0..s.len() {
            let idx = (s.as_bytes()[i] - b'a') as usize;
            if d[idx] != 0 && distance[idx] as u8 != i as u8 - d[idx] {
                return false;
            }
            d[idx] = i as u8 + 1;
        }
        true
    }

    pub fn count_days_together_2409(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        const DAYS_IN_MONTH: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let count_days = |d: &str| -> u16 {
            let month = d[..2].parse::<usize>().unwrap();
            (0..month - 1).fold(0u16, |mut days, i| {
                days += DAYS_IN_MONTH[i] as u16;
                days
            }) + d[3..].parse::<u16>().unwrap()
        };

        let (a1, l1) = (count_days(&arrive_alice), count_days(&leave_alice));
        let (a2, l2) = (count_days(&arrive_bob), count_days(&leave_bob));

        (1..=365).fold(0i32, |mut res, d| {
            if a1 <= d && a2 <= d && d <= l1 && d <= l2 {
                res += 1;
            }
            res
        })
    }

    pub fn sort_people_2418_1(names: Vec<String>, mut heights: Vec<i32>) -> Vec<String> {
        use std::collections::HashMap;
        let map_name: HashMap<i32, String> =
            heights.iter().zip(names).map(|(h, v)| (*h, v)).collect();
        heights.sort_by(|a, b| b.cmp(a));

        heights
            .iter()
            .map(|h| map_name[h].clone())
            .collect::<Vec<String>>()
    }

    pub fn sort_people_2418_2(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut idx: Vec<usize> = (0..names.len()).collect();
        idx.sort_by(|i, j| heights[*j].cmp(&heights[*i]));
        idx.iter()
            .map(|i| names[*i].clone())
            .collect::<Vec<String>>()
    }

    pub fn equal_frequency_2423(word: String) -> bool {
        use std::collections::HashSet;
        let mut freq = word.as_bytes().iter().fold([0u8; 26], |mut c, b| {
            c[(*b - b'a') as usize] += 1;
            c
        });

        let check_valid = |freq: &[u8; 26]| -> bool {
            freq.iter()
                .filter(|c| **c != 0)
                .cloned()
                .collect::<HashSet<u8>>()
                .len()
                == 1
        };

        for i in 0..freq.len() {
            freq[i] -= 1;
            if check_valid(&freq) {
                return true;
            }
            freq[i] += 1;
        }

        false
    }

    pub fn count_time_2437(time: String) -> i32 {
        let mut res = 1;

        if time.as_bytes()[1] == b'?' {
            match time.as_bytes()[0] {
                b'0' | b'1' => res *= 10,
                b'2' => res *= 4,
                b'?' => res *= 24,
                _ => (),
            }
        } else {
            if time.as_bytes()[0] == b'?' {
                res *= if time.as_bytes()[1] >= b'4' { 2 } else { 3 };
            }
        }

        if time.as_bytes()[4] == b'?' {
            match time.as_bytes()[3] {
                b'?' => res *= 60,
                _ => res *= 10,
            }
        } else {
            if time.as_bytes()[3] == b'?' {
                res *= 6;
            }
        }

        res
    }

    pub fn have_conflict_2446(event1: Vec<String>, event2: Vec<String>) -> bool {
        fn cmp_time(t1: &str, t2: &str, i: usize) -> bool {
            if i == 5 {
                return true;
            }
            if t1.as_bytes()[i] == t2.as_bytes()[i] {
                return cmp_time(t1, t2, i + 1);
            }
            t1.as_bytes()[i] < t2.as_bytes()[i]
        }

        cmp_time(&event2[0], &event1[1], 0) && cmp_time(&event1[0], &event2[1], 0)
    }

    pub fn odd_strings_2451(words: Vec<String>) -> String {
        let dif = words
            .iter()
            .map(|w| {
                (1..w.len())
                    .map(|i| w.as_bytes()[i] as i8 - w.as_bytes()[i - 1] as i8)
                    .collect::<Vec<i8>>()
            })
            .collect::<Vec<Vec<i8>>>();

        if dif[0] != dif[1] && dif[1] == dif[2] {
            return words[0].clone();
        }

        for i in 1..dif.len() {
            if dif[i] != dif[i - 1] {
                return words[i].clone();
            }
        }

        String::new()
    }

    pub fn is_circular_sentence_2490(sentence: String) -> bool {
        if sentence.as_bytes()[0] != *sentence.as_bytes().last().unwrap() {
            return false;
        }

        for i in 1..sentence.len() {
            if sentence.as_bytes()[i] == b' ' {
                if sentence.as_bytes()[i - 1] != sentence.as_bytes()[i + 1] {
                    return false;
                }
            }
        }

        true
    }

    pub fn maximum_value_2496(strs: Vec<String>) -> i32 {
        strs.into_iter()
            .map(|w| {
                let mut res = 0;
                for i in 0..w.len() {
                    if w.as_bytes()[i].is_ascii_alphabetic() {
                        return w.len() as i32;
                    }
                    res = res * 10 + (w.as_bytes()[i] - b'0') as i32;
                }
                res
            })
            .max()
            .unwrap()
    }

    pub fn similar_pairs_2506_1(words: Vec<String>) -> i32 {
        // burte force + BTreeSet: 16ms => Bad code
        use std::collections::BTreeSet;

        let cs: Vec<BTreeSet<char>> = words
            .iter()
            .map(|w| w.chars().collect::<BTreeSet<char>>())
            .collect();

        let mut res = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if cs[i] == cs[j] {
                    res += 1;
                }
            }
        }

        res
    }

    pub fn similar_pairs_2506_2(words: Vec<String>) -> i32 {
        use std::collections::{BTreeSet, HashMap};

        let mut map: HashMap<String, i32> = HashMap::new();
        words.iter().for_each(|w| {
            let s: String = w.chars().collect::<BTreeSet<char>>().into_iter().collect();
            map.entry(s).and_modify(|c| *c += 1).or_insert(1);
        });

        map.values().fold(0, |mut res, cnt| {
            res += cnt * (cnt - 1) / 2;
            res
        })
    }

    pub fn closet_target_2515(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let mut res = usize::MAX;
        for i in 0..words.len() {
            if words[i] == target {
                let diff = i.abs_diff(start_index as usize);
                res = res.min(diff.min(words.len() - diff));
            }
        }
        res as i32
    }

    pub fn vowles_strings_2586(words: Vec<String>, left: i32, right: i32) -> i32 {
        const VOWELS: &str = "aeiou";
        ((left as usize)..=(right as usize))
            .filter(|i| {
                VOWELS.find(words[*i].as_bytes()[0] as char).is_some()
                    && VOWELS
                        .find(*words[*i].as_bytes().last().unwrap() as char)
                        .is_some()
            })
            .count() as i32
    }

    pub fn find_the_longest_balanced_substring_2609(s: String) -> i32 {
        let mut sub_str = String::from("01");
        let mut flag = false;
        while s.find(&sub_str).is_some() {
            flag = true;
            sub_str = String::from("0") + &sub_str + "1";
        }

        if !flag {
            0
        } else {
            sub_str.len() as i32 - 2
        }
    }

    pub fn count_seniors_2678(details: Vec<String>) -> i32 {
        details
            .into_iter()
            .filter(|w| (w.as_bytes()[11] - b'0') * 10 + (w.as_bytes()[12] - b'0') > 60)
            .count() as i32
    }

    pub fn min_length_2696(s: String) -> i32 {
        let mut res: Vec<u8> = vec![];

        for b in s.as_bytes() {
            if res.len() != 0 {
                let last_byte = res.last().unwrap();
                if (*b == b'B' && *last_byte == b'A') || (*b == b'C' && *last_byte == b'D') {
                    res.pop();
                    continue;
                }
            }
            res.push(*b);
        }

        unsafe { String::from_utf8_unchecked(res).len() as i32 }
    }

    pub fn make_smallest_palindrome_2697(s: String) -> String {
        let n = s.len();
        let mut bytes: Vec<u8> = s.into_bytes();
        for i in 0..bytes.len() / 2 {
            if bytes[i] != bytes[n - 1 - i] {
                let min = bytes[i].min(bytes[n - 1 - i]);
                bytes[i] = min;
                bytes[n - 1 - i] = min;
            }
        }
        unsafe { String::from_utf8_unchecked(bytes) }
    }

    pub fn remove_trailing_zeros_2710(mut num: String) -> String {
        while *num.as_bytes().last().unwrap() == b'0' {
            num.remove(num.len() - 1);
        }
        num
    }

    pub fn minimized_string_length_2716(s: String) -> i32 {
        use std::collections::HashSet;
        s.chars().collect::<HashSet<char>>().len() as i32
    }

    pub fn maximum_number_of_string_pairs_2744(words: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let set: HashSet<String> = words.into_iter().collect();
        set.iter().fold(0, |mut res, s| {
            let reversed_str: String = s.chars().rev().collect();
            if reversed_str != *s && set.contains(&reversed_str) {
                res += 1;
            }
            res
        }) / 2
    }

    pub fn split_words_by_separator_2788_1(words: Vec<String>, separator: char) -> Vec<String> {
        words.iter().fold(vec![], |mut res: Vec<String>, w| {
            let mut tmp = String::new();
            w.chars().for_each(|c| {
                if c == separator {
                    if tmp.len() != 0 {
                        res.push(tmp.clone());
                        tmp.clear();
                    }
                } else {
                    tmp.push(c);
                }
            });

            if tmp.len() != 0 {
                res.push(tmp.clone());
            }
            res
        })
    }

    pub fn split_words_by_separator_2788_2(words: Vec<String>, separator: char) -> Vec<String> {
        words.iter().fold(vec![], |mut res: Vec<String>, w| {
            res.extend(w.split(separator).filter_map(|s| {
                if s.len() != 0 {
                    Some(s.to_string())
                } else {
                    None
                }
            }));
            res
        })
    }

    pub fn final_string_2810(s: String) -> String {
        let mut res: Vec<u8> = vec![];
        let reverse = |arr: &mut Vec<u8>| {
            let n = arr.len();
            for i in 0..n / 2 {
                arr.swap(i, n - 1 - i);
            }
        };

        for i in 0..s.len() {
            if s.as_bytes()[i] == b'i' {
                if res.len() > 1 {
                    reverse(&mut res);
                }
                continue;
            }

            res.push(s.as_bytes()[i]);
        }

        unsafe { String::from_utf8_unchecked(res) }
    }

    pub fn is_acronym_2828(words: Vec<String>, s: String) -> bool {
        words.iter().fold(String::new(), |mut acc: String, w| {
            acc.push(w.as_bytes()[0] as char);
            acc
        }) == s
    }

    pub fn furthest_distance_from_origin_2833(moves: String) -> i32 {
        let mut cnt = 0;
        (moves.chars().fold(0, |mut res, c| {
            match c {
                'L' => res -= 1,
                'R' => res += 1,
                _ => cnt += 1,
            };
            res
        }) as i32)
            .abs()
            + cnt
    }

    pub fn can_be_equal_2839(s1: String, s2: String) -> bool {
        if s1 == s2 {
            return true;
        }

        let mut s1 = s1.into_bytes();
        let s2 = s2.into_bytes();

        s1.swap(0, 2);
        if s1 == s2 {
            return true;
        }

        s1.swap(1, 3);
        if s1 == s2 {
            return true;
        }

        s1.swap(0, 2);
        if s1 == s2 {
            return true;
        }

        return false;
    }

    pub fn maximum_odd_binary_number_2864(s: String) -> String {
        let cnt_one = s.as_bytes().iter().filter(|b| **b == b'1').count();
        String::from("1").repeat(cnt_one - 1) + &String::from("0").repeat(s.len() - cnt_one) + "1"
    }

    pub fn get_longest_subsequence_2900(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut flag = -1;
        groups
            .iter()
            .enumerate()
            .fold(vec![], |mut res: Vec<String>, (i, b)| {
                if *b != flag {
                    flag = *b;
                    res.push(words[i].clone());
                }
                res
            })
    }

    pub fn find_minimum_operations_2937(s1: String, s2: String, s3: String) -> i32 {
        let mut max_len = 0;
        for i in 0..s1.len().min(s2.len().min(s3.len())) {
            if s1.as_bytes()[i] != s2.as_bytes()[i]
                || s2.as_bytes()[i] != s3.as_bytes()[i]
                || s1.as_bytes()[i] != s3.as_bytes()[i]
            {
                break;
            }
            max_len += 1;
        }

        if max_len == 0 {
            return -1;
        }

        (s1.len() + s2.len() + s3.len() - 3 * max_len) as i32
    }

    pub fn find_words_containing_2942(words: Vec<String>, x: char) -> Vec<i32> {
        (0..words.len())
            .filter_map(|i| {
                if words[i].contains(x) {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn minimum_pushes_3014(word: String) -> i32 {
        let k = word.len() / 8;
        (k * (k + 1) * 4 + (word.len() % 8) * (k + 1)) as i32
    }

    pub fn count_key_changes_3019(s: String) -> i32 {
        let s = s.to_lowercase();
        (0..s.len()).fold(0, |mut res, i| {
            if i != 0 && s.as_bytes()[i] != s.as_bytes()[i - 1] {
                res += 1
            }
            res
        })
    }

    pub fn count_prefix_suffix_pairs_3042(words: Vec<String>) -> i32 {
        (0..words.len()).fold(0, |res, i| {
            res + (i + 1..words.len())
                .filter(|j| {
                    if words[i].len() > words[*j].len() {
                        return false;
                    }
                    words[*j].starts_with(words[i].as_str())
                        && words[*j].ends_with(words[i].as_str())
                })
                .count()
        }) as i32
    }

    pub fn is_substring_present_3083(s: String) -> bool {
        let reversed_str: String = s.chars().rev().collect();
        for i in 0..s.len() - 1 {
            if reversed_str.contains(&s[i..i + 2]) {
                return true;
            }
        }
        false
    }

    pub fn maximum_length_substring_3090(s: String) -> i32 {
        let mut freq = [0u8; 26];
        let mut start = 0;
        let mut res = 0;
        for i in 0..s.len() {
            freq[(s.as_bytes()[i] - b'a') as usize] += 1;
            while freq[(s.as_bytes()[i] - b'a') as usize] == 3 {
                freq[(s.as_bytes()[start] - b'a') as usize] -= 1;
                start += 1;
            }
            res = res.max(i - start + 1);
        }
        res as i32
    }

    pub fn score_of_string_3110(s: String) -> i32 {
        (1..s.len()).fold(0, |res, i| {
            res + s.as_bytes()[i].abs_diff(s.as_bytes()[i - 1]) as i32
        })
    }

    pub fn find_latest_time_3114(s: String) -> String {
        let mut bytes: Vec<u8> = s.into_bytes();
        if bytes[0] == b'?' {
            if bytes[1] == b'?' || bytes[1] < b'2' {
                bytes[0] = b'1';
            } else {
                bytes[0] = b'0';
            }
        }

        if bytes[1] == b'?' {
            bytes[1] = if bytes[0] == b'0' { b'9' } else { b'1' };
        }

        if bytes[3] == b'?' {
            bytes[3] = b'5';
        }

        if bytes[4] == b'?' {
            bytes[4] = b'9';
        }

        unsafe { String::from_utf8_unchecked(bytes) }
    }

    pub fn number_of_special_chars_3120(word: String) -> i32 {
        let mut up_freq = [0u8; 26];
        let mut lo_freq = [0u8; 26];

        word.as_bytes().iter().for_each(|b| {
            if b.is_ascii_uppercase() {
                up_freq[(*b - b'A') as usize] += 1;
            } else {
                lo_freq[(*b - b'a') as usize] += 1;
            }
        });

        (0..26)
            .filter(|&i| up_freq[i] != 0 && lo_freq[i] != 0)
            .count() as i32
    }

    pub fn is_valid_3136(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }

        let mut cnt_vowel = 0;
        let mut cnt_consonant = 0;

        for b in word.as_bytes().iter() {
            if matches!(*b, b'@' | b'#' | b'$') {
                return false;
            }

            if b.is_ascii_alphabetic() {
                if matches!(b.to_ascii_lowercase(), b'a' | b'e' | b'u' | b'o' | b'i') {
                    cnt_vowel += 1;
                } else {
                    cnt_consonant += 1;
                }
            }
        }

        cnt_consonant != 0 && cnt_vowel != 0
    }

    pub fn find_permutation_difference_3146(s: String, t: String) -> i32 {
        let mut s_occ = [-1i8; 26];
        let mut t_occ = [-1i8; 26];

        (0..s.len()).for_each(|i| {
            s_occ[(s.as_bytes()[i] - b'a') as usize] = i as i8;
            t_occ[(t.as_bytes()[i] - b'a') as usize] = i as i8;
        });

        (0..26).fold(0, |res, i| {
            if s_occ[i] != -1 {
                res + s_occ[i].abs_diff(t_occ[i]) as i32
            } else {
                res
            }
        })
    }

    pub fn minimum_chairs_3168(s: String) -> i32 {
        let mut res = 0;
        let mut curr = 0;
        s.chars().for_each(|c| {
            if c == 'E' {
                curr += 1;
                res = res.max(curr);
            } else {
                curr -= 1;
            }
        });

        res
    }

    pub fn clear_digits_3174(s: String) -> String {
        let mut res: Vec<u8> = vec![];
        s.as_bytes().iter().for_each(|b| {
            if b.is_ascii_digit() && res.len() != 0 {
                res.pop();
                return;
            }
            res.push(*b);
        });
        unsafe { String::from_utf8_unchecked(res) }
    }

    pub fn get_encrypted_string_3210(s: String, k: i32) -> String {
        (0..s.len()).fold(String::new(), |mut res, i| {
            res.push(s.as_bytes()[(i + k as usize) % s.len()] as char);
            res
        })
    }

    pub fn get_smallest_string_3216(s: String) -> String {
        let mut bytes = s.into_bytes();
        for i in 1..bytes.len() {
            if (bytes[i] + bytes[i - 1]) % 2 == 0 && bytes[i] < bytes[i - 1] {
                bytes.swap(i - 1, i);
                break;
            }
        }
        unsafe { String::from_utf8_unchecked(bytes) }
    }

    pub fn final_position_of_snake_3248(n: i32, commands: Vec<String>) -> i32 {
        commands.iter().fold(0, |res, s| match s.as_str() {
            "UP" => res - n,
            "DOWN" => res + n,
            "RIGHT" => res + 1,
            _ => res - 1,
        })
    }
}
