# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def pseudoPalindromicPaths (self, root: Optional[TreeNode]) -> int:
        count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        def f(node):
            count[node.val] += 1
            if node.left or node.right:
                if node.left: f(node.left)
                if node.right: f(node.right)
            else:
                奇数count = 0
                for x in count[1:]:
                    if x % 2 == 1: 奇数count += 1
                if 奇数count < 2:
                    count[0] += 1
            count[node.val] -= 1
        f(root)
        return count[0]