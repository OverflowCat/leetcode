# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def goodNodes(self, root: TreeNode) -> int:
        self.sum = 0
        self.f(root, root.val)
        return self.sum
    def f(self, node: TreeNode, maxv: int):
        if maxv <= node.val:
            self.sum += 1
        maxv = max(node.val, maxv)
        if node.left is not None:
            self.f(node.left, maxv)
        if node.right is not None:
            self.f(node.right, maxv)
        # print("sum of", node.val, "is", sum)
