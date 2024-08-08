package ac

import "hashirama/leetcode/tmp"

func isSameTree(p *tmp.TreeNode, q *tmp.TreeNode) bool {
	if p == nil && q == nil {
		return true
	}
	if p != nil && q != nil {
		return p.Val == q.Val && isSameTree(p.Right, q.Right) && isSameTree(p.Left, q.Left)
	}
	return false
}
