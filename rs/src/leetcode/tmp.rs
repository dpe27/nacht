#[allow(dead_code)]
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

impl Solution {
    pub fn is_anagram_242(s: String, t: String) -> bool {}

    pub fn word_pattern_290(pattern: String, s: String) -> bool {}

    pub fn reverse_string_344(s: &mut Vec<char>) {}

    pub fn reverse_vowels_345(s: String) -> String {}

    pub fn can_construct_383(ransom_note: String, magazine: String) -> bool {}

    pub fn first_uniq_char_387(s: String) -> i32 {}

    pub fn find_the_difference_389(s: String, t: String) -> char {}

    pub fn longest_palindrome_409(s: String) -> i32 {}

    pub fn fizz_buzz_412(n: i32) -> Vec<String> {}

    pub fn add_strings_415(num1: String, num2: String) -> String {}

    pub fn count_segments_434(s: String) -> i32 {}

    pub fn repeated_substring_pattern_459(s: String) -> bool {}

    pub fn license_key_formatting_482(s: String, k: i32) -> String {}

    pub fn find_words_500(words: Vec<String>) -> Vec<String> {}

    pub fn detect_capital_use_520(word: String) -> bool {}

    pub fn find_lu_slength_521(a: String, b: String) -> i32 {}

    pub fn reverse_str_541(s: String, k: i32) -> String {}

    pub fn check_record_551(s: String) -> bool {}

    pub fn reverse_words_557(s: String) -> String {}

    pub fn find_restaurant_599(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {}

    pub fn judge_circle_657(moves: String) -> bool {}

    pub fn valid_palindrome_680(s: String) -> bool {}

    pub fn count_binary_substrings_696(s: String) -> i32 {}

    pub fn shortest_completing_word_748(license_plate: String, words: Vec<String>) -> String {}

    pub fn num_jewels_in_stones_771(jewels: String, stones: String) -> i32 {}

    pub fn rotate_string_796(s: String, goal: String) -> bool {}

    pub fn unique_morse_representations_804(words: Vec<String>) -> i32 {}

    pub fn number_of_lines_806(widths: Vec<i32>, s: String) -> Vec<i32> {}

    pub fn most_common_word_819(paragraph: String, banned: Vec<String>) -> String {}

    pub fn shortest_to_char_821(s: String, c: char) -> Vec<i32> {}

    pub fn to_goat_latin_824(sentence: String) -> String {}

    pub fn large_group_positions_830(s: String) -> Vec<Vec<i32>> {}

    pub fn backspace_compare_844(s: String, t: String) -> bool {}

    pub fn buddy_strings_859(s: String, goal: String) -> bool {}

    pub fn uncommon_from_sentences_884(s1: String, s2: String) -> Vec<String> {}

    pub fn reverse_only_letters_917(s: String) -> String {}

    pub fn is_long_pressed_name_925(name: String, typed: String) -> bool {}

    pub fn num_unique_emails_929(emails: Vec<String>) -> i32 {}

    pub fn di_string_match_942(s: String) -> Vec<i32> {}

    pub fn min_deletion_size_944(strs: Vec<String>) -> i32 {}

    pub fn is_alien_sorted_953(words: Vec<String>, order: String) -> bool {}

    pub fn common_chars_1002(words: Vec<String>) -> Vec<String> {}
}
