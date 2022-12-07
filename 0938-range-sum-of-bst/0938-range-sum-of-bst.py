class Solution:
    def rangeSumBST(self, root: TreeNode, low: int, high: int) -> int:
        sum = 0
        def f(node):
            if node is None: return
            if node.val < low:
                f(node.right)
            elif node.val > high:
                f(node.left)
            else:
                nonlocal sum
                sum += node.val
                f(node.left)
                f(node.right)
        f(root)
        return sum
