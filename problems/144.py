"""
BINARY TREE PREORDER TRAVERSAL
"""

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
  def preorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
    if not root:
      return []
    cur = root
    stack = []
    li = []
    stack.append(cur)
    while stack != []:
      #print([x.val for x in stack])
      cur = stack.pop()
      li.append(cur.val)
      if (cur.right):
        stack.append(cur.right)
      if (cur.left):
        stack.append(cur.left)
      #print([x for x in li])
    return li
