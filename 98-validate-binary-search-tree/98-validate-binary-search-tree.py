# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isValidBST(self, root: Optional[TreeNode], minnode=None, maxnode=None) -> bool:
        if root is None:
            return True
        # validate
        if minnode is not None and root.val <= minnode.val:
            return False
        if maxnode is not None and root.val >= maxnode.val:
            return False
        return self.isValidBST(root.left, minnode, root) and self.isValidBST(root.right, root, maxnode)
