package ac

import "hashirama/leetcode/tmp"

func sortedArrayToBST(nums []int) *tmp.TreeNode {
	var build func(start, end int) *tmp.TreeNode
	build = func(start, end int) *tmp.TreeNode {
		if start > end {
			return nil
		}
		mid := start + (end-start)/2
		root := &tmp.TreeNode{Val: nums[mid]}
		root.Left = build(start, mid)
		root.Right = build(mid+1, end)
		return root
	}
	return build(0, len(nums)-1)
}
