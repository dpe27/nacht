package ac

import "hashirama/leetcode/tmp"

func isSymmetric(root *tmp.TreeNode) bool {
	var check func(*tmp.TreeNode, *tmp.TreeNode) bool
	check = func(left *tmp.TreeNode, right *tmp.TreeNode) bool {
		if left == nil && right == nil {
			return true
		}
		if left == nil || right == nil {
			return false
		}
		return left.Val == right.Val && check(left.Left, right.Right) && check(left.Right, right.Left)
	}
	return check(root.Left, root.Right)
}
