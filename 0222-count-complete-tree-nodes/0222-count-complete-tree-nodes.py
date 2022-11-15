class Solution:
    def countNodes(self, root: Optional[TreeNode]) -> int:
        l, r = root, root
        lc, rc = 0, 0
        while l:
            l = l.left
            lc += 1
        while r:
            r = r.right
            rc += 1
        if lc == rc: return 2 ** lc - 1
        return 1 + self.countNodes(root.left) + self.countNodes(root.right)
