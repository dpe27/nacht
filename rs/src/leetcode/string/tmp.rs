#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    //pub fn count_k_constraint_substrings_3258(s: String, k: i32) -> i32 {}

    pub fn check_two_chessboards_3274(coordinate1: String, coordinate2: String) -> bool {
        (coordinate1.as_bytes()[0]
            + coordinate1.as_bytes()[1]
            + coordinate2.as_bytes()[0]
            + coordinate2.as_bytes()[1])
            % 2
            == 0
    }

    //pub fn convert_date_to_binary_3280(date: String) -> String {}

    pub fn possible_string_count_3330_1(word: String) -> i32 {
        let mut res = 0;
        let mut cnt = 0;
        let mut prev = word.as_bytes()[0];

        for i in 0..word.len() {
            if word.as_bytes()[i] != prev {
                res += cnt - 1;
                cnt = 0;
                prev = word.as_bytes()[i];
            }
            cnt += 1;
        }
        res + cnt
    }

    pub fn possible_string_count_3330_2(word: String) -> i32 {
        word.into_bytes()
            .windows(2)
            .map(|w| (w[0] == w[1]) as i32)
            .sum::<i32>()
            + 1
    }

    pub fn is_balanced_3340(num: String) -> bool {
        let mut s1 = 0;
        let mut s2 = 0;
        (0..num.len()).for_each(|i| {
            let c = num.as_bytes()[i] - b'0';
            if i % 2 == 0 {
                s1 += c;
            } else {
                s2 += c;
            }
        });
        s1 == s2
    }

    pub fn has_match_3407(s: String, p: String) -> bool {
        if s.len() < p.len() - 1 {
            return false;
        }
        let k = p.find('*').unwrap();

        if k == 0 {
            s.rfind(&p[k + 1..]).is_some()
        } else if k == p.len() - 1 {
            s.find(&p[..k]).is_some()
        } else {
            let i = s.find(&p[..k]);
            let j = s.rfind(&p[k + 1..]);
            i.is_some() && j.is_some() && i.unwrap() + k - 1 != j.unwrap()
        }
    }
}
