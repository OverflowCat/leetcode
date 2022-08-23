# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

all_nodes = {}

class Solution:
        
    def amountOfTime(self, root, start: int) -> int:
        def addToAllNodes(curr):
            if curr.val not in all_nodes:
                all_nodes[curr.val] = {}
            if curr.left:
                all_nodes[curr.val]["l"] = curr.left
                if curr.left not in all_nodes:
                    all_nodes[curr.left.val] = {}
                all_nodes[curr.left]["p"] = curr.val
                addToAllNodes(curr.left)
            if curr.right:
                all_nodes[curr.val]["r"] = curr.right
                if curr.right not in all_nodes:
                    all_nodes[curr.right.val] = {}
                all_nodes[curr.right]["p"] = curr.val
                addToAllNodes(curr.right)
        addToAllNodes(root)

        def getTime(curr, time):
            
        