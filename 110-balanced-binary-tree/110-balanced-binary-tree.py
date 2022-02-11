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
            if lheight != -1 and rheight != -1:
                if rheight == lheight:
                    return rheight + 1
                if rheight - lheight == 1:
                    return rheight + 1
                if rheight - lheight == -1:
                    return lheight + 1
            return -1
        return height(root) != -1