/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */

func max(a, b int) int {
    if a > b {
        return a
    }
    return b
}

func diameterOfBinaryTree(root *TreeNode) int {
    maxv := 0
    f(root, &maxv)
    return maxv
}

    func f(root *TreeNode, maxv *int) int {
        left := 0
        right := 0
        if root.Left != nil {
            left = f(root.Left, maxv)
        }
        if root.Right != nil {
            right = f(root.Right, maxv)
        }
        *maxv = max(left + right, *maxv)
        return 1 + max(left, right)
    }
