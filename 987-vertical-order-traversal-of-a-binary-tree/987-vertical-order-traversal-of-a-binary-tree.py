# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def verticalTraversal(self, root: Optional[TreeNode]) -> List[List[int]]:
        m = defaultdict(lambda: defaultdict(list))
        def f(node, x, y):
            m[x][y].append(node.val)
            if node.left is not None:
                f(node.left, x - 1, y + 1)
            if node.right is not None:
                f(node.right, x + 1, y + 1)
        f(root, 0, 0)
        m = sorted(m.items())
        res = []
        for k, col in m:
            c = []
            # print("col", col)
            for k, row in sorted(col.items()):
                # print("row", row)
                c += sorted(row)
            res.append(c)
        return res
        