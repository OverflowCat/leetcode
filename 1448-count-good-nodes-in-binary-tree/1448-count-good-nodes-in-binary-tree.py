# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        def f(node: TreeNode, maxv: int) -> int:
            sum = 0
            if maxv <= node.val:
                sum = 1
            maxv = max(node.val, maxv)
            if node.left is not None:
                sum += f(node.left, maxv)
            if node.right is not None:
                sum += f(node.right, maxv)
            # print("sum of", node.val, "is", sum)
            return sum
        return f(root, root.val)
