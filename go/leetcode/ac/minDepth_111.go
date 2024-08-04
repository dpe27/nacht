package ac

import "hashirama/leetcode/tmp"

func minDepth(root *tmp.TreeNode) int {
	if root == nil {
		return 0
	}
	left := minDepth(root.Left)
	right := minDepth(root.Right)
	if left == 0 && right != 0 {
		return right + 1
	}
	if left != 0 && right == 0 {
		return left + 1
	}
	return min(left, right) + 1
}
