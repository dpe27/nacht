package ac

import "hashirama/leetcode/tmp"

func isValidBST(root *tmp.TreeNode) bool {
	var inorderArr []int
	var dfs func(root *tmp.TreeNode) bool
	dfs = func(root *tmp.TreeNode) bool {
		if root.Left != nil {
			if root.Left.Val >= root.Val {
				return false
			}
			if !dfs(root.Left) {
				return false
			}
		}
		if len(inorderArr) > 0 && root.Val <= inorderArr[len(inorderArr)-1] {
			return false
		}
		inorderArr = append(inorderArr, root.Val)
		if root.Right != nil {
			if root.Right.Val <= root.Val {
				return false
			}
			if !dfs(root.Right) {
				return false
			}
		}
		return true
	}
	return dfs(root)
}
