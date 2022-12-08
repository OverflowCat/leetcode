class Solution:
    def leafSimilar(self, root1: Optional[TreeNode], root2: Optional[TreeNode]) -> bool:
        seq1, seq2 = [], []
        def f1(node):
            if node.left is not None or node.right is not None:
                if node.left  is not None: f1(node.left)
                if node.right is not None: f1(node.right)
            else:
                seq1.append(node.val)
        def f2(node):
            if node.left is not None or node.right is not None:
                if node.left  is not None: f2(node.left)
                if node.right is not None: f2(node.right)
            else:
                seq2.append(node.val)
        f1(root1)
        f2(root2)
        print(seq1, seq2)
        return seq1 == seq2