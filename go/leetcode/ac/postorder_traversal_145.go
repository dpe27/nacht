package ac

import "hashirama/leetcode/tmp"

func postorderTraversal(root *tmp.TreeNode) []int {
	var res []int
	var dfs func(root *tmp.TreeNode)
	dfs = func(root *tmp.TreeNode) {
		if root == nil {
			return
		}
		dfs(root.Left)
		dfs(root.Right)
		res = append(res, root.Val)
	}
	dfs(root)
	return res
}
