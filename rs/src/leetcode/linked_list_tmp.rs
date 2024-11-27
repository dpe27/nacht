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
pub struct Solution;

#[allow(dead_code)]
impl Solution {}
