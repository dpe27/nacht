package ac

func pathSum(root *TreeNode, targetSum int) [][]int {
	var res [][]int
	var dfs func(root *TreeNode, target int, path []int)
	dfs = func(root *TreeNode, target int, path []int) {
		if root == nil {
			return
		}
		path = append(path, root.Val)
		if root.Left == nil && root.Right == nil && target-root.Val == 0 {
			res = append(res, append([]int{}, path...))
		}
		dfs(root.Left, target-root.Val, path)
		dfs(root.Right, target-root.Val, path)
		path = path[:len(path)-1]
	}
	dfs(root, targetSum, []int{})
	return res
}
