# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if root is None:
            return None
        def rev(node):
            if node.left is not None and node.right is not None:
                swap = node.left
                node.left = node.right
                node.right = swap
                rev(node.left)
                rev(node.right)
            elif node.left is not None:
                node.right = node.left
                node.left = None
                rev(node.right)
            elif node.right is not None:
                node.left = node.right
                node.right = None
                rev(node.left)
        rev(root)
        return root