class Solution:
    def pruneTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        def f(node) -> bool:
            res = node.val == 1
            if node.left:
                if f(node.left): res = True
                else: node.left = None
            if node.right:
                if f(node.right): res = True
                else: node.right = None
            return res
        return root if f(root) else None