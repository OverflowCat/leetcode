class Solution:
    def pseudoPalindromicPaths (self, root: Optional[TreeNode]) -> int:
        global count, res
        count, res = 0, 0
        def f(node):
            global count, res
            count ^= 1 << node.val
            if node.left or node.right:
                if node.left: f(node.left)
                if node.right: f(node.right)
            else:
                if count & count - 1 == 0: res += 1
            count ^= 1 << node.val
        f(root)
        return res
