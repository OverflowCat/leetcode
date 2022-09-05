"""
# Definition for a Node.
class Node:
    def __init__(self, val=None, children=None):
        self.val = val
        self.children = children
"""

class Solution:
    def levelOrder(self, root: 'Node') -> List[List[int]]:
        res = defaultdict(list)
        def f(node, lv: int):
            if node is None:
                return
            res[lv].append(node.val)
            for x in node.children:
                f(x, lv + 1)
        f(root, 0)
        return res.values()