package ac

import "hashirama/leetcode/tmp"

func SortedListToBST(head *tmp.ListNode) *tmp.TreeNode {
	if head == nil {
		return nil
	}
	var arrNode []int
	for head != nil {
		arrNode = append(arrNode, head.Val)
		head = head.Next
	}

	var build func(start, end int) *tmp.TreeNode
	build = func(start, end int) *tmp.TreeNode {
		if start > end {
			return nil
		}
		midIdx := start + (end-start)/2
		root := &tmp.TreeNode{Val: arrNode[midIdx]}
		root.Left = build(start, midIdx-1)
		root.Right = build(midIdx+1, end)
		return root
	}
	return build(0, len(arrNode)-1)
}
