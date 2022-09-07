# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def tree2str(self, node: Optional[TreeNode]) -> str:
        res = str(node.val)
        if node.left:
            res += '(' + self.tree2str(node.left) + ')'
            if node.right: res += '(' + self.tree2str(node.right) + ')'
        else:
            if node.right: res += '()(' + self.tree2str(node.right) + ')'
        return res
