"""
BINARY TREE INORDER TRAVERSAL
"""

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = righ

"""
def p(node):
  if node is None:
    return None
  else:
    return node.val
"""

class Solution:
  def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
    
    if not root:
      return []
    cur = root
    stack = []
    li = []
    stack.append(cur)
    while stack != []:
      #print([p(x) for x in stack])
      cur = stack.pop()
      if cur is not None:
        if (cur.right):
          stack.append(cur.right)
        stack.append(cur)
        stack.append(None)
        if (cur.left):
          stack.append(cur.left)
      else:
        cur = stack.pop()
        li.append(cur.val)
      #print(li)
    return li