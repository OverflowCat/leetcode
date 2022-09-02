# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def averageOfLevels(self, root: Optional[TreeNode]) -> List[float]:
        sums = defaultdict(lambda: 0)
        counts = defaultdict(lambda: 0)
        def calculate(node: TreeNode, lv: int):
            sums[lv] += node.val
            counts[lv] += 1
            if node.left is not None:
                calculate(node.left, lv + 1)
            if node.right is not None:
                calculate(node.right, lv + 1)
        calculate(root, 0)
        print(sums, counts)
        return [sums[i]/counts[i] if counts[i] != 0 else 0 for i in range(len(sums))]
