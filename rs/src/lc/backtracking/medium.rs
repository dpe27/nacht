#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn letter_combinations_17(digits: String) -> Vec<String> {
        if digits.len() == 0 {
            return Vec::new();
        }

        let phone = vec!["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
        let mut res: Vec<String> = vec![];

        fn backtrack(
            combination: String,
            next_digit: &str,
            res: &mut Vec<String>,
            phone: &Vec<&str>,
        ) {
            if next_digit.is_empty() {
                res.push(combination);
            } else {
                let letters = phone[(next_digit.as_bytes()[0] - b'2') as usize];
                letters.chars().for_each(|c| {
                    backtrack(
                        format!("{}{}", combination, c),
                        &next_digit[1..],
                        res,
                        phone,
                    )
                });
            }
        }

        backtrack(String::new(), &digits, &mut res, &phone);
        res
    }

    pub fn generate_parenthesis_22(n: i32) -> Vec<String> {
        let mut res = vec![];
        fn backtrack(open: i32, close: i32, sub: String, res: &mut Vec<String>) {
            if open > close || open < 0 || close < 0 {
                return;
            }

            if open == 0 && close == 0 {
                res.push(sub);
                return;
            }

            backtrack(open - 1, close, format!("{}(", sub), res);
            backtrack(open, close - 1, format!("{})", sub), res);
        }

        backtrack(n, n, String::new(), &mut res);
        res
    }

    pub fn combination_sum_39(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        fn backtrack(
            idx: usize,
            total: i32,
            acc: &mut Vec<i32>,
            target: i32,
            candidates: &Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if total == target {
                res.push(acc.clone());
                return;
            }

            if idx >= candidates.len() || total > target {
                return;
            }

            acc.push(candidates[idx]);
            backtrack(idx, total + candidates[idx], acc, target, candidates, res);
            acc.pop();
            backtrack(idx + 1, total, acc, target, candidates, res);
        }

        backtrack(0, 0, &mut Vec::new(), target, &candidates, &mut res);
        res
    }

    pub fn combination_sum2_40(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut res = vec![];
        fn backtrack(
            idx: usize,
            total: i32,
            acc: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
            target: i32,
            candidates: &Vec<i32>,
        ) {
            if total == target {
                res.push(acc.clone());
            }

            if idx == candidates.len() || total > target {
                return;
            }

            for i in idx..candidates.len() {
                if i != idx && candidates[i] == candidates[i - 1] {
                    continue;
                }
                acc.push(candidates[i]);
                backtrack(i + 1, total + candidates[i], acc, res, target, candidates);
                acc.pop();
            }
        }

        backtrack(0, 0, &mut Vec::new(), &mut res, target, &candidates);
        res
    }

    pub fn permute_46(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        fn backtrack(acc: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, nums: &[i32]) {
            if nums.len() == 0 {
                res.push(acc.clone());
                return;
            }

            for i in 0..nums.len() {
                acc.push(nums[i]);
                backtrack(acc, res, &[&nums[..i], &nums[i + 1..]].concat());
                acc.pop();
            }
        }
        backtrack(&mut Vec::new(), &mut res, &nums);
        res
    }

    pub fn permute_unique_47(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut used = vec![false; nums.len()];
        let mut res = vec![];

        fn backtrack(
            acc: &mut Vec<i32>,
            used: &mut Vec<bool>,
            nums: &Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if acc.len() == nums.len() {
                res.push(acc.clone());
                return;
            }

            for i in 0..nums.len() {
                if used[i] || (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
                    continue;
                }

                used[i] = true;
                acc.push(nums[i]);

                backtrack(acc, used, nums, res);

                used[i] = false;
                acc.pop();
            }
        }

        backtrack(&mut Vec::new(), &mut used, &nums, &mut res);
        res
    }

    pub fn combine_77(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        fn backtrack(acc: &mut Vec<i32>, idx: i32, k: i32, n: i32, res: &mut Vec<Vec<i32>>) {
            if k > n {
                return;
            }
            if acc.len() == k as usize {
                res.push(acc.clone());
                return;
            }

            for i in idx..=n {
                acc.push(i);
                backtrack(acc, i + 1, k, n, res);
                acc.pop();
            }
        }
        backtrack(&mut Vec::new(), 1, k, n, &mut res);
        res
    }

    pub fn subsets_78(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        fn backtrack(acc: &mut Vec<i32>, idx: usize, res: &mut Vec<Vec<i32>>, nums: &Vec<i32>) {
            res.push(acc.clone());
            for i in idx..nums.len() {
                acc.push(nums[i]);
                backtrack(acc, i + 1, res, nums);
                acc.pop();
            }
        }
        backtrack(&mut Vec::new(), 0, &mut res, &nums);
        res
    }

    pub fn exist_79(board: Vec<Vec<char>>, word: String) -> bool {
        const DIRECTIONS: [(i8, i8); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let mut closed = vec![vec![false; board[0].len()]; board.len()];

        fn backtrack(
            closed: &mut Vec<Vec<bool>>,
            idx: i8,
            row: i8,
            col: i8,
            board: &Vec<Vec<char>>,
            word: &str,
        ) -> bool {
            if board[row as usize][col as usize] as u8 != word.as_bytes()[idx as usize] {
                return false;
            }

            if idx as usize == word.len() - 1 {
                return true;
            }

            for &d in DIRECTIONS.iter() {
                let (new_row, new_col) = (row + d.0, col + d.1);
                if new_row >= 0
                    && new_row < board.len() as i8
                    && new_col >= 0
                    && new_col < board[0].len() as i8
                    && !closed[new_row as usize][new_col as usize]
                {
                    closed[new_row as usize][new_col as usize] = true;
                    if backtrack(closed, idx + 1, new_row, new_col, board, word) {
                        return true;
                    }
                    closed[new_row as usize][new_col as usize] = false;
                }
            }

            return false;
        }

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if board[i][j] as u8 == word.as_bytes()[0] {
                    closed[i][j] = true;
                    if backtrack(&mut closed, 0, i as i8, j as i8, &board, &word) {
                        return true;
                    }
                    closed[i][j] = false;
                }
            }
        }

        false
    }

    pub fn subsets_with_dup_90(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res = vec![];
        fn backtrack(pos: usize, acc: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, nums: &Vec<i32>) {
            res.push(acc.clone());
            for i in pos..nums.len() {
                if i > pos && nums[i] == nums[i - 1] {
                    continue;
                }

                acc.push(nums[i]);
                backtrack(i + 1, acc, res, nums);
                acc.pop();
            }
        }

        backtrack(0, &mut Vec::new(), &mut res, &nums);
        res
    }

    pub fn restore_ip_addresses_93(s: String) -> Vec<String> {
        fn is_valid(segment: &str) -> bool {
            if segment.len() > 1 && segment.starts_with('0') {
                return false;
            }

            segment.parse::<u16>().unwrap() <= 255
        }

        fn solve(start: usize, acc: &mut Vec<String>, s: &str, res: &mut Vec<String>) {
            if start == s.len() && acc.len() == 8 {
                res.push(acc[..acc.len() - 1].concat());
                return;
            }

            for end in start + 1..=s.len().min(start + 3) {
                let segment = &s[start..end];
                if is_valid(segment) {
                    acc.extend(vec![segment.to_string(), ".".to_string()]);
                    solve(end, acc, s, res);
                    acc.truncate(acc.len().saturating_sub(2));
                }
            }
        }

        let mut res = vec![];
        solve(0, &mut vec![], &s, &mut res);
        return res;
    }

    pub fn partition_131(s: String) -> Vec<Vec<String>> {
        let mut res = vec![];
        if s.len() == 0 {
            return res;
        }

        fn is_palindrome(s: &str) -> bool {
            for i in 0..s.len() / 2 {
                if s.as_bytes()[i] != s.as_bytes()[s.len() - i - 1] {
                    return false;
                }
            }
            true
        }

        fn solve(s: &str, step: &mut Vec<String>, res: &mut Vec<Vec<String>>) {
            if s.len() == 0 {
                res.push(step.clone());
                return;
            }

            for i in 1..=s.len() {
                let tmp = &s[..i];
                if !is_palindrome(tmp) {
                    continue;
                }

                step.push(tmp.to_string());
                solve(&s[i..], step, res);
                step.pop();
            }
        }

        solve(&s, &mut vec![], &mut res);
        res
    }

    pub fn combination_sum3_216(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut used = [false; 9];
        let mut res = vec![];

        fn solve(
            i: usize,
            sum: i32,
            step: &mut Vec<i32>,
            used: &mut [bool; 9],
            res: &mut Vec<Vec<i32>>,
            k: i32,
            n: i32,
        ) {
            if step.len() > k as usize || sum > n {
                return;
            }

            if step.len() == k as usize && sum == n {
                res.push(step.clone());
                return;
            }

            for ele in i..=9 {
                if !used[ele - 1] {
                    step.push(ele as i32);
                    used[ele - 1] = true;
                    solve(ele + 1, sum + ele as i32, step, used, res, k, n);
                    step.pop();
                    used[ele - 1] = false;
                }
            }
        }

        solve(1, 0, &mut vec![], &mut used, &mut res, k, n);
        res
    }

    pub fn is_additive_number_306(num: String) -> bool {
        if num.len() < 3 {
            return false;
        }

        fn add_large_integer(s1: &str, s2: &str) -> String {
            let mut res: Vec<u8> = vec![];
            let mut i = s1.len();
            let mut j = s2.len();
            let mut rem = 0;

            while i > 0 || j > 0 {
                let sum: u8;
                if i > 0 && j > 0 {
                    i -= 1;
                    j -= 1;
                    sum = s1.as_bytes()[i] + s2.as_bytes()[j] - 2 * b'0' + rem;
                } else if i > 0 {
                    i -= 1;
                    sum = s1.as_bytes()[i] - b'0' + rem;
                } else {
                    j -= 1;
                    sum = s2.as_bytes()[j] - b'0' + rem;
                }

                res.push(sum % 10 + b'0');
                rem = sum / 10;
            }

            if rem != 0 {
                res.push(b'1');
            }
            res.reverse();

            unsafe { String::from_utf8_unchecked(res) }
        }

        fn is_valid(s1: &str, s2: &str, acc: String, num: &str) -> bool {
            if (s1.len() > 1 && s1.starts_with('0'))
                || (s2.len() > 1 && s2.starts_with('0'))
                || acc.len() > num.len()
                || acc != num[..acc.len()]
            {
                return false;
            }

            if acc == num {
                return true;
            }

            let sum = add_large_integer(s1, s2);
            is_valid(s2, &sum, acc + &sum, num)
        }

        for i in 1..num.len() - 1 {
            for j in 0..i {
                let s1 = &num[..j + 1];
                let s2 = &num[j + 1..i + 1];
                if is_valid(s1, s2, num[..i + 1].to_string(), &num) {
                    return true;
                }
            }
        }

        false
    }

    pub fn count_numbers_with_unique_digits_357(n: i32) -> i32 {
        let mut res = 0;
        let mut used = [false; 10];
        fn solve(l: u8, used: &mut [bool; 10], res: &mut i32, n: i32) {
            if l >= n as u8 {
                return;
            }
            for i in 0..=9 {
                if i == 0 && l == 0 {
                    continue;
                }
                if !used[i] {
                    *res += 1;
                    used[i] = true;
                    solve(l + 1, used, res, n);
                    used[i] = false;
                }
            }
        }

        solve(0, &mut used, &mut res, n);
        res + 1 // include 0
    }

    pub fn makesquare_473(matchsticks: Vec<i32>) -> bool {
        // wtf leetcode: TLE ??? (with backtracking tag) -> DP
        fn solve(idx: usize, sides_length: &mut [i32; 4], matches: &Vec<i32>) -> bool {
            if idx == matches.len() {
                return sides_length[0] == sides_length[1]
                    && sides_length[1] == sides_length[2]
                    && sides_length[2] == sides_length[3];
            }
            for i in 0..4 {
                sides_length[i] += matches[idx];
                if solve(idx + 1, sides_length, matches) {
                    return true;
                }
                sides_length[i] -= matches[idx];
            }
            false
        }
        solve(0, &mut [0i32; 4], &matchsticks)
    }

    pub fn find_subsequences_491(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        fn solve(
            idx: usize,
            prev: i32,
            acc: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
            nums: &Vec<i32>,
        ) {
            if idx == nums.len() {
                if acc.len() > 1 {
                    res.push(acc.clone());
                }
                return;
            }

            if acc.is_empty() || (!acc.is_empty() && nums[idx] >= *acc.last().unwrap()) {
                acc.push(nums[idx]);
                solve(idx + 1, nums[idx], acc, res, nums);
                acc.pop();
            }

            if nums[idx] == prev {
                return;
            }

            solve(idx + 1, prev, acc, res, nums);
        }
        solve(0, i32::MIN, &mut vec![], &mut res, &nums);
        res
    }

    pub fn find_target_sum_ways_494(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = 0;
        fn solve(idx: usize, sum: i32, res: &mut i32, nums: &Vec<i32>, target: i32) {
            if idx == nums.len() {
                if sum == target {
                    *res += 1;
                }
                return;
            }

            for k in [-1, 1] {
                solve(idx + 1, sum + k * nums[idx], res, nums, target);
            }
        }
        solve(0, 0, &mut res, &nums, target);
        res
    }

    pub fn count_arrangement_526(n: i32) -> i32 {
        fn solve(k: u8, res: &mut i32, used: &mut u16, n: u8) {
            if k > n {
                *res += 1;
                return;
            }

            for i in 1..=n {
                if *used & (1 << i) == 0 && (i % k == 0 || k % i == 0) {
                    *used |= 1 << i;
                    solve(k + 1, res, used, n);
                    *used &= !(1 << i);
                }
            }
        }

        let mut res = 0;
        solve(1, &mut res, &mut 0u16, n as u8);
        return res;
    }

    pub fn shopping_offers_638(price: Vec<i32>, special: Vec<Vec<i32>>, needs: Vec<i32>) -> i32 {
        fn calc_total_price(price: &Vec<i32>, needs: &Vec<i32>) -> i32 {
            (0..needs.len()).fold(0, |total, i| total + needs[i] * price[i])
        }

        fn solve(idx: usize, price: &Vec<i32>, special: &Vec<Vec<i32>>, needs: &Vec<i32>) -> i32 {
            let mut total = calc_total_price(price, needs);

            for i in idx..special.len() {
                let mut flag = false;
                let mut new_needs = vec![];
                for j in 0..needs.len() {
                    if special[i][j] > needs[j] {
                        flag = true;
                        break;
                    }
                    new_needs.push(needs[j] - special[i][j]);
                }

                if !flag {
                    total = total
                        .min(*special[i].last().unwrap() + solve(i, price, special, &new_needs))
                }
            }

            return total;
        }

        solve(0, &price, &special, &needs)
    }

    pub fn can_partition_k_subsets_698_1(mut nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }

        let target = sum / k;
        let mut used = vec![false; nums.len()];
        nums.sort_unstable_by(|a, b| b.cmp(a));

        fn solve(
            idx: usize,
            cur_sum: i32,
            target: i32,
            k: i32,
            used: &mut Vec<bool>,
            nums: &Vec<i32>,
        ) -> bool {
            if k == 1 {
                return true;
            }

            if cur_sum == target {
                return solve(0, 0, target, k - 1, used, nums);
            }

            for i in idx..nums.len() {
                if i > 0 && !used[i - 1] && nums[i - 1] == nums[i] {
                    continue;
                }

                if !used[i] && cur_sum + nums[i] <= target {
                    used[i] = true;
                    if solve(i + 1, cur_sum + nums[i], target, k, used, nums) {
                        return true;
                    }
                    used[i] = false;
                }
            }
            false
        }
        solve(0, 0, target, k, &mut used, &nums)
    }

    pub fn can_partition_k_subsets_698_2(mut nums: Vec<i32>, k: i32) -> bool {
        let sum: i32 = nums.iter().sum();
        if sum % k != 0 {
            return false;
        }

        nums.sort_unstable_by(|a, b| b.cmp(a));
        fn solve(idx: usize, target: i32, subsets: &mut Vec<i32>, nums: &Vec<i32>) -> bool {
            if idx == nums.len() {
                return true;
            }

            for i in 0..subsets.len() {
                if subsets[i] + nums[idx] <= target {
                    subsets[i] += nums[idx];
                    if solve(idx + 1, target, subsets, nums) {
                        return true;
                    }
                    subsets[i] -= nums[idx];

                    if subsets[i] == 0 {
                        break; // dont understand :((
                    }
                }
            }

            false
        }

        solve(0, sum / k, &mut vec![0; k as usize], &nums)
    }

    pub fn letter_case_permutation_784(s: String) -> Vec<String> {
        let bytes = s.as_bytes();
        let mut res = vec![];

        fn solve(pos: usize, acc: &mut Vec<u8>, bytes: &[u8], res: &mut Vec<String>) {
            if pos == bytes.len() {
                res.push(unsafe { String::from_utf8_unchecked(acc.clone()) });
                return;
            }

            let b = bytes[pos];
            if b.is_ascii_alphabetic() {
                acc.push(b.to_ascii_uppercase());
                solve(pos + 1, acc, bytes, res);
                acc.pop();

                acc.push(b.to_ascii_lowercase());
                solve(pos + 1, acc, bytes, res);
                acc.pop();
            } else {
                acc.push(b);
                solve(pos + 1, acc, bytes, res);
                acc.pop();
            }
        }

        solve(0, &mut vec![], bytes, &mut res);
        res
    }

    pub fn all_paths_source_target_797(_graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        vec![] // DFS, BFS, graph
    }

    pub fn ambiguous_coordinates_816(s: String) -> Vec<String> {
        let bytes = &s.as_bytes()[1..s.len() - 1];
        let mut res = vec![];

        fn convert(s: &[u8]) -> Vec<String> {
            if s.len() > 1 && s[0] == b'0' && s[s.len() - 1] == b'0' {
                return Vec::new();
            }

            unsafe {
                if s[0] == b'0' {
                    if s.len() == 1 {
                        return vec!["0".to_string()];
                    }

                    return vec![format!(
                        "0.{}",
                        String::from_utf8_unchecked(s[1..].to_vec())
                    )];
                }

                let mut res = vec![String::from_utf8_unchecked(s.to_vec())];
                if s[s.len() - 1] == b'0' {
                    return res;
                }

                for i in 1..s.len() {
                    res.push(format!(
                        "{}.{}",
                        String::from_utf8_unchecked(s[..i].to_vec()),
                        String::from_utf8_unchecked(s[i..].to_vec()),
                    ))
                }
                return res;
            }
        }

        for i in 1..bytes.len() {
            let str_arr_1 = convert(&bytes[..i]);
            let str_arr_2 = convert(&bytes[i..]);

            for s1 in str_arr_1.iter() {
                for s2 in str_arr_2.iter() {
                    res.push(format!("({}, {})", s1, s2));
                }
            }
        }

        res
    }

    pub fn split_into_fibonacci_842(num: String) -> Vec<i32> {
        let mut res = vec![];

        fn is_valid(s: &str) -> bool {
            s.len() != 0 && !(s.len() > 1 && s.as_bytes()[0] == b'0') && s.parse::<i32>().is_ok()
        }

        fn solve(s1: &str, s2: &str, acc: &mut String, res: &mut Vec<i32>, num: &str) -> bool {
            if acc.len() > num.len() || num[..acc.len()] != *acc {
                return false;
            }

            if num == *acc {
                return true;
            }

            if is_valid(s2) {
                let sum = s1.parse::<i32>().unwrap() + s2.parse::<i32>().unwrap();
                let sum_str = sum.to_string();
                res.push(sum);
                acc.push_str(&sum_str);
                if solve(s2, &sum_str, acc, res, num) {
                    return true;
                }
                res.pop();
            }

            false
        }

        'outer: for i in 1..num.len() - 1 {
            for j in 0..i {
                let s1 = &num[..j + 1];
                let s2 = &num[j + 1..i + 1];

                if is_valid(s1) && is_valid(s2) {
                    res.extend([s1.parse::<i32>().unwrap(), s2.parse::<i32>().unwrap()]);
                    if solve(s1, s2, &mut num[..i + 1].to_string(), &mut res, &num) {
                        break 'outer;
                    }
                    res.clear();
                }
            }
        }

        res
    }

    pub fn nums_same_consec_diff_967(n: i32, k: i32) -> Vec<i32> {
        let mut res = vec![];

        fn solve(pos: u8, acc: i32, prev: u8, res: &mut Vec<i32>, n: u8, k: u8) {
            if pos == n {
                res.push(acc);
                return;
            }

            if pos == 0 {
                for d in 1..=9 {
                    solve(pos + 1, d as i32, d, res, n, k);
                }
            } else {
                if k == 0 {
                    solve(pos + 1, acc * 10 + prev as i32, prev, res, n, k);
                    return;
                }

                if prev + k <= 9 {
                    solve(pos + 1, acc * 10 + (prev + k) as i32, prev + k, res, n, k);
                }

                if prev >= k {
                    solve(pos + 1, acc * 10 + (prev - k) as i32, prev - k, res, n, k);
                }
            }
        }

        solve(0, 0, 0, &mut res, n as u8, k as u8);
        res
    }

    pub fn num_tile_possibilities_1079_1(tiles: String) -> i32 {
        let mut res = 0;
        let mut used = vec![false; tiles.len()];
        let mut tiles = tiles.into_bytes();

        tiles.sort_unstable();

        fn solve(acc: &mut Vec<u8>, res: &mut i32, used: &mut Vec<bool>, tiles: &[u8]) {
            if acc.len() == tiles.len() {
                return;
            }

            for i in 0..tiles.len() {
                if i != 0 && tiles[i - 1] == tiles[i] && !used[i - 1] {
                    continue;
                }

                if !used[i] {
                    used[i] = true;
                    *res += 1;
                    acc.push(tiles[i]);
                    solve(acc, res, used, tiles);
                    acc.pop();
                    used[i] = false;
                }
            }
        }

        solve(&mut vec![], &mut res, &mut used, &tiles);
        res
    }

    pub fn num_tile_possibilities_1079_2(tiles: String) -> i32 {
        let mut freq = [0; 26];
        tiles
            .as_bytes()
            .iter()
            .for_each(|b| freq[(*b - b'A') as usize] += 1);

        fn dfs(freq: &mut [i32; 26]) -> i32 {
            let mut total = 0;
            for i in 0..26 {
                if freq[i] == 0 {
                    continue;
                }

                total += 1;
                freq[i] -= 1;
                total += dfs(freq);
                freq[i] += 1;
            }

            return total;
        }

        dfs(&mut freq)
    }

    pub fn get_maximum_gold_1219(mut grid: Vec<Vec<i32>>) -> i32 {
        const DIRECTIONS: [i8; 5] = [0, 1, 0, -1, 0];
        let mut res = 0;

        fn solve(x: i8, y: i8, grid: &mut Vec<Vec<i32>>) -> i32 {
            if (x < 0 || x >= grid.len() as i8)
                || (y < 0 || y >= grid[0].len() as i8)
                || grid[x as usize][y as usize] == 0
            {
                return 0;
            }

            let curr_cell = grid[x as usize][y as usize];
            grid[x as usize][y as usize] = 0;

            let mut max_gold = 0;
            (0..4).for_each(|i| {
                max_gold = max_gold.max(solve(x + DIRECTIONS[i], y + DIRECTIONS[i + 1], grid))
            });

            grid[x as usize][y as usize] = curr_cell;
            return curr_cell + max_gold;
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                res = res.max(solve(i as i8, j as i8, &mut grid));
            }
        }

        res
    }

    pub fn get_happy_string_1415(n: i32, k: i32) -> String {
        const CHARS: [u8; 3] = [b'a', b'b', b'c'];

        fn solve(acc: &mut Vec<u8>, str_arr: &mut Vec<String>, n: usize) {
            if acc.len() == n {
                unsafe {
                    str_arr.push(String::from_utf8_unchecked(acc.clone()));
                }
                return;
            }

            for b in CHARS.iter() {
                if acc.is_empty() || *b != *acc.last().unwrap() {
                    acc.push(*b);
                    solve(acc, str_arr, n);
                    acc.pop();
                }
            }
        }

        let mut str_arr = vec![];
        solve(&mut vec![], &mut str_arr, n as usize);

        str_arr.sort_unstable();
        if str_arr.len() < k as usize {
            return String::new();
        }

        str_arr[k as usize - 1].clone()
    }

    pub fn max_unique_split_1593(s: String) -> i32 {
        use std::collections::HashSet;

        fn solve(start: usize, str_set: &mut HashSet<String>, s: &str) -> i32 {
            if start == s.len() {
                return 0;
            }

            let mut max_splits = 0;
            for end in start + 1..=s.len() {
                if !str_set.contains(&s[start..end]) {
                    str_set.insert(s[start..end].to_string());
                    max_splits = max_splits.max(1 + solve(end, str_set, s));
                    str_set.remove(&s[start..end]);
                }
            }

            return max_splits;
        }

        solve(0, &mut HashSet::new(), &s)
    }

    pub fn construct_distanced_sequence_1718(n: i32) -> Vec<i32> {
        let mut used = 0i32;

        fn solve(idx: usize, used: &mut i32, n: i32, res: &mut Vec<i32>) -> bool {
            if idx == res.len() as usize {
                return true;
            }

            if res[idx] != 0 {
                return solve(idx + 1, used, n, res);
            }

            for i in (1..=n).rev() {
                if *used & (1 << (i - 1)) == 0 {
                    *used |= 1 << (i - 1);

                    if i == 1 {
                        res[idx] = 1;

                        if solve(idx + 1, used, n, res) {
                            return true;
                        }

                        res[idx] = 0;
                    } else if idx + (i as usize) < res.len() && res[idx + i as usize] == 0 {
                        res[idx] = i;
                        res[idx + i as usize] = i;

                        if solve(idx + 1, used, n, res) {
                            return true;
                        }

                        res[idx] = 0;
                        res[idx + i as usize] = 0
                    }

                    *used &= !(1 << (i - 1));
                }
            }

            return false;
        }

        let mut res = vec![0i32; 2 * n as usize - 1];
        solve(0, &mut used, n, &mut res);
        return res;
    }

    pub fn closest_cost_1774(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        fn solve(idx: usize, total: i32, res: &mut i32, topping_costs: &Vec<i32>, target: i32) {
            if target.abs_diff(total) < target.abs_diff(*res)
                || (target.abs_diff(total) == target.abs_diff(*res) && total < *res)
            {
                *res = total;
            }

            if idx == topping_costs.len() || total >= target {
                return;
            }

            solve(idx + 1, total, res, topping_costs, target);
            solve(
                idx + 1,
                total + topping_costs[idx],
                res,
                topping_costs,
                target,
            );
            solve(
                idx + 1,
                total + topping_costs[idx] * 2,
                res,
                topping_costs,
                target,
            );
        }

        let mut res = base_costs[0];
        base_costs
            .iter()
            .for_each(|b| solve(0, *b, &mut res, &topping_costs, target));
        res
    }

    pub fn split_string_1849(s: String) -> bool {
        let s = s.trim_start_matches('0');
        if s.len() <= 1 {
            return false;
        }

        fn solve(start: usize, prev: u64, s: &str) -> bool {
            if start == s.len() {
                return true;
            }

            for end in start + 1..=s.len() {
                if s[start..end].parse::<u64>().unwrap() == prev - 1 {
                    if solve(end, prev - 1, s) {
                        return true;
                    }
                }
            }

            return false;
        }

        for end in 1..=s.len() / 2 + 1 {
            if solve(end, s[0..end].parse::<u64>().unwrap(), &s) && end != s.len() {
                return true;
            }
        }

        return false;
    }

    pub fn max_compatibility_sum_1947(students: Vec<Vec<i32>>, mentors: Vec<Vec<i32>>) -> i32 {
        fn calc_compatibility_score(x: u8, y: u8, n: u8) -> u8 {
            n - (x ^ y).count_ones() as u8
        }

        fn solve(
            curr: usize,
            sum: u8,
            men_used: &mut u8,
            res: &mut u8,
            students: &Vec<u8>,
            mentors: &Vec<u8>,
            n: u8,
        ) {
            if curr == students.len() {
                *res = sum.max(*res);
                return;
            }

            for i in 0..mentors.len() {
                if *men_used & (1 << i) as u8 == 0 {
                    *men_used |= 1 << i;
                    solve(
                        curr + 1,
                        sum + calc_compatibility_score(students[curr], mentors[i], n),
                        men_used,
                        res,
                        students,
                        mentors,
                        n,
                    );
                    *men_used &= !(1 << i);
                }
            }
        }

        let n = students[0].len();
        let map_to_bit = |answers: Vec<Vec<i32>>| -> Vec<u8> {
            answers
                .iter()
                .map(|ans| {
                    let mut res = 0u8;
                    for i in 0..ans.len() {
                        res |= ans[i] as u8;
                        if i != ans.len() - 1 {
                            res <<= 1;
                        }
                    }
                    return res;
                })
                .collect()
        };

        let students = map_to_bit(students);
        let mentors = map_to_bit(mentors);

        let mut res = 0;
        solve(0, 0, &mut 0u8, &mut res, &students, &mentors, n as u8);
        res as i32
    }

    pub fn find_different_binary_string_1980(nums: Vec<String>) -> String {
        // Diagonalization Method
        nums.iter()
            .enumerate()
            .fold(String::new(), |mut s, (idx, k)| {
                let bit = if k.as_bytes()[idx] == b'0' { '1' } else { '0' };
                s.push(bit);
                return s;
            })
    }

    pub fn count_max_or_subsets_2044(nums: Vec<i32>) -> i32 {
        fn solve(idx: usize, curr_or: i32, res: &mut i32, max_or: i32, nums: &Vec<i32>) {
            if curr_or == max_or {
                *res += 1;
            }

            if idx == nums.len() {
                return;
            }

            (idx..nums.len()).for_each(|i| solve(i + 1, curr_or | nums[i], res, max_or, nums));
        }

        let max_or = nums.iter().fold(0, |acc, k| acc | *k);
        let mut res = 0;
        solve(0, 0, &mut res, max_or, &nums);
        return res;
    }

    pub fn maximum_even_split_2178(final_sum: i64) -> Vec<i64> {
        if final_sum & 1 == 1 {
            return vec![];
        }

        let mut res: Vec<i64> = vec![];
        let mut cur_sum = 0;
        let mut k = 2;

        while cur_sum + k <= final_sum {
            res.push(k);
            cur_sum += k;
            k += 2;
        }

        *res.last_mut().unwrap() += final_sum - cur_sum;
        return res;
    }

    pub fn maximum_bob_points_2212(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        fn solve(
            idx: usize,
            cur_score: i32,
            max_score: &mut i32,
            num_arrows: i32,
            bob_arrows: &mut Vec<i32>,
            alice_arrows: &Vec<i32>,
            res: &mut Vec<i32>,
        ) {
            if idx == alice_arrows.len() || num_arrows == 0 {
                if cur_score > *max_score {
                    *max_score = cur_score;
                    *res = bob_arrows.clone();

                    if num_arrows > 0 {
                        res[0] += num_arrows;
                    }
                }
                return;
            }

            if num_arrows >= alice_arrows[idx] + 1 {
                bob_arrows[idx] = alice_arrows[idx] + 1;
                solve(
                    idx + 1,
                    cur_score + idx as i32,
                    max_score,
                    num_arrows - alice_arrows[idx] - 1,
                    bob_arrows,
                    alice_arrows,
                    res,
                );
                bob_arrows[idx] = 0;
            }

            solve(
                idx + 1,
                cur_score,
                max_score,
                num_arrows,
                bob_arrows,
                alice_arrows,
                res,
            );
        }

        let mut res = vec![];
        solve(
            0,
            0,
            &mut 0i32,
            num_arrows,
            &mut vec![0i32; alice_arrows.len()],
            &alice_arrows,
            &mut res,
        );
        return res;
    }

    pub fn smallest_number_2375(pattern: String) -> String {
        fn dfs(idx: usize, used: u16, prev: u8, res: &mut Vec<u8>, pattern: &str) -> bool {
            if idx == pattern.len() + 1 {
                return true;
            }

            let increase = if pattern.as_bytes()[idx - 1] == b'I' {
                true
            } else {
                false
            };

            for i in 1..=9 {
                if used & (1 << i) == 0 {
                    if idx > 0 && ((increase && i < prev) || (!increase && i > prev)) {
                        continue;
                    }

                    res.push(i + b'0');
                    if dfs(idx + 1, used | (1 << i), i, res, pattern) {
                        return true;
                    }
                    res.pop();
                }
            }

            return false;
        }

        let mut res = vec![];
        for i in 1..=9 {
            res.push(i + b'0');
            if dfs(1, 1 << i, i, &mut res, &pattern) {
                break;
            }
            res.pop();
        }
        unsafe { String::from_utf8_unchecked(res) }
    }

    pub fn beautiful_subsets_2597(mut nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        fn solve(idx: usize, freq: &mut HashMap<i32, u8>, nums: &Vec<i32>, k: i32) -> i32 {
            if idx == nums.len() {
                return 1;
            }

            let mut res = solve(idx + 1, freq, nums, k);
            if freq.get(&(nums[idx] - k)).is_none() {
                freq.entry(nums[idx]).and_modify(|c| *c += 1).or_insert(1);
                res += solve(idx + 1, freq, nums, k);
                let c = freq.get_mut(&nums[idx]).unwrap();
                *c -= 1;
                if *c == 0 {
                    freq.remove(&nums[idx]);
                }
            }

            return res;
        }

        nums.sort_unstable();
        solve(0, &mut HashMap::new(), &nums, k) - 1
    }

    pub fn punishment_number_2698(n: i32) -> i32 {
        fn is_partitionable(start: usize, sum: i32, square: &str, k: i32) -> bool {
            if sum == k && start == square.len() {
                return true;
            }

            for end in start + 1..=square.len() {
                if is_partitionable(
                    end,
                    sum + square[start..end].parse::<i32>().unwrap(),
                    square,
                    k,
                ) {
                    return true;
                }
            }

            return false;
        }

        (1..=n).fold(0, |sum, k| {
            if is_partitionable(0, 0, &k.pow(2).to_string(), k) {
                return sum + k.pow(2);
            }
            sum
        })
    }

    pub fn valid_strings_3211(n: i32) -> Vec<String> {
        fn solve(idx: usize, acc: u32, n: usize, res: &mut Vec<String>) {
            if idx == n {
                res.push(format!("{:0n$b}", acc, n = n));
                return;
            }

            solve(idx + 1, acc << 1 | 1, n, res);
            if acc & 1 == 1 || idx == 0 {
                solve(idx + 1, acc << 1, n, res);
            }
        }

        let mut res = vec![];
        solve(0, 0, n as usize, &mut res);
        res
    }
}

#[allow(dead_code, non_camel_case_types)]
struct CombinationIterator_1286 {
    s: String,
    len: usize,
    mask: u32,
}

#[allow(dead_code)]
impl CombinationIterator_1286 {
    fn new(characters: String, combination_length: i32) -> Self {
        let mask = (1 << characters.len()) - 1;
        Self {
            s: characters,
            len: combination_length as usize,
            mask,
        }
    }

    fn next(&mut self) -> String {
        while self.mask > 0 && self.mask.count_ones() != self.len as u32 {
            self.mask -= 1;
        }

        let mut out = String::new();
        for (i, c) in self.s.chars().enumerate() {
            if self.mask & (1 << (self.s.len() - i - 1)) != 0 {
                out.push(c);
            }
        }

        self.mask -= 1;
        out
    }

    fn has_next(&mut self) -> bool {
        while self.mask > 0 && self.mask.count_ones() != self.len as u32 {
            self.mask -= 1;
        }
        self.mask > 0
    }
}
