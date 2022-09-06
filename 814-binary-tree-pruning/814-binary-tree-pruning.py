class Solution:
    def pruneTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        if not root: return
        root.left, root.right = self.pruneTree(root.left), self.pruneTree(root.right)
        if root.val == 1 or root.left or root.right: return root
