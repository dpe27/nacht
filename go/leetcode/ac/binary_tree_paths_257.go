package ac

import (
	"strconv"
	"strings"
)

func binaryTreePaths(root *TreeNode) []string {
	var res []string
	var buildPath func(root *TreeNode, path []string)
	buildPath = func(root *TreeNode, path []string) {
		if root == nil {
			return
		}
		path = append(path, strconv.Itoa(root.Val))
		if root.Left == nil && root.Right == nil {
			res = append(res, strings.Join(path, "->"))
			return
		}
		buildPath(root.Left, path)
		buildPath(root.Right, path)
		path = path[:len(path)-1]
	}
	buildPath(root, []string{})
	return res
}
