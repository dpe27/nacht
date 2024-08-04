package ac

import "hashirama/leetcode/tmp"

func isBalanced(root *tmp.TreeNode) bool {
	var findDepth func(root *tmp.TreeNode) int
	findDepth = func(root *tmp.TreeNode) int {
		if root == nil {
			return 0
		}
		left := findDepth(root.Left)
		right := findDepth(root.Right)
		if left == -1 || right == -1 || left-right < -1 || left-right > 1 {
			return -1
		}
		return max(left, right) + 1
	}
	if findDepth(root) == -1 {
		return false
	}
	return true
}
