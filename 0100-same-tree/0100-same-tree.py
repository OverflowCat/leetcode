class Solution:
    def isSameTree(self, p: Optional[TreeNode], q: Optional[TreeNode]) -> bool:
        stack=[]
        stack.append((p, q))
        while len(stack) != 0:
            a, b = stack.pop()
            if a is None and b is None:
                continue
            if a is None or b is None:
                return False
            if a.val != b.val:
                return False
            stack.append((a.left, b.left))
            stack.append((a.right, b.right))
        return True
