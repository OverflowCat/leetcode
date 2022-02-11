# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def isBalanced(self, root: Optional[TreeNode]) -> bool:
        def height(node: Optional[TreeNode]) -> int:
            if node is None:
                return 0
            lheight = height(node.left)
            rheight = height(node.right)
            if lheight != -1 and rheight != -1 and abs(rheight - lheight) <= 1:
                return max(rheight, lheight) + 1
            raise
        try:
            height(root)
            return True
        except:
            return False