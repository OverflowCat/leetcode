class Solution:
    def pathSum(self, root: Optional[TreeNode], targetSum: int) -> List[List[int]]:
        if root:
            diff = targetSum - root.val
            if root.left or root.right:
                return [[root.val] + li for li in self.pathSum(root.left, diff) + self.pathSum(root.right, diff)]
            if root.val == targetSum: return [[root.val]]
        return []
