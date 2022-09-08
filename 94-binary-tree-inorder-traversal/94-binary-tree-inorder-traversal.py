# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        res = []
        def f(node):
            if node.left: f(node.left)
            res.append(node.val)
            if node.right: f(node.right)
        if root is None: return
        f(root)
        return res