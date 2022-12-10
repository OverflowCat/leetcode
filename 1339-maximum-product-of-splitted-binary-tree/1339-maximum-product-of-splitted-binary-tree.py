
class Solution:
    def maxProduct(self, root: TreeNode) -> int:
        res = 0
        alle = 0
        def s(node) -> int:
            if node is None: return 0
            left = s(node.left)
            right = s(node.right)
            this = left + right + node.val
            nonlocal res, alle
            res = max(res, this * (alle - this))
            return this
        alle = s(root)
        s(root)
        return res % 1000000007