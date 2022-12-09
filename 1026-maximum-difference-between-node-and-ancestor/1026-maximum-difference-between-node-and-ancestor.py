class Solution:
    def maxAncestorDiff(self, root: TreeNode) -> int:
        max_diff = 0
        def f(node: TreeNode) -> (int, int):
            nonlocal max_diff
            if node.left is None and node.right is None:
                # diff = 0
                return (node.val, node.val)
            if node.left is None:
                min_v, max_v = f(node.right)
                diff = max(abs(node.val - min_v), abs(node.val - max_v))
                if diff > max_diff: max_diff = diff
                return min(node.val, min_v), max(node.val, max_v)
            if node.right is None:
                min_v, max_v = f(node.left)
                diff = max(abs(node.val - min_v), abs(node.val - max_v))
                if diff > max_diff: max_diff = diff
                return min(node.val, min_v), max(node.val, max_v)
            min_v_l, max_v_l = f(node.left)
            min_v_r, max_v_r = f(node.right)
            min_v, max_v = min(min_v_l, min_v_r), max(max_v_l, max_v_r)
            diff = max(abs(node.val - min_v), abs(node.val - max_v))
            if diff > max_diff: max_diff = diff
            return min(node.val, min_v), max(node.val, max_v)
        f(root)
        return max_diff