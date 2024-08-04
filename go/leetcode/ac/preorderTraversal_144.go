package ac

import "hashirama/leetcode/tmp"

func preorderTraversal(root *tmp.TreeNode) []int {
	var res []int
	var dfs func(root *tmp.TreeNode)
	dfs = func(root *tmp.TreeNode) {
		if root == nil {
			return
		}
		res = append(res, root.Val)
		dfs(root.Left)
		dfs(root.Right)
	}
	dfs(root)
	return res
}
