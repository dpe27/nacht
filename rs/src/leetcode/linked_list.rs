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
}
