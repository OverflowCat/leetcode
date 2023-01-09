class Solution:
  def preorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
    if not root: return []
    li = []
    stack = [root]
    cur = root
    while stack != []:
      cur = stack.pop()
      li.append(cur.val)
      if (cur.right): stack.append(cur.right)
      if (cur.left):  stack.append(cur.left)
    return li