package ac

import "hashirama/leetcode/tmp"

func hasPathSum(root *tmp.TreeNode, targetSum int) bool {
	if root == nil {
		return false
	}
	if root.Right == nil && root.Left == nil && targetSum-root.Val == 0 {
		return true
	}
	return hasPathSum(root.Left, targetSum-root.Val) || hasPathSum(root.Right, targetSum-root.Val)
}
