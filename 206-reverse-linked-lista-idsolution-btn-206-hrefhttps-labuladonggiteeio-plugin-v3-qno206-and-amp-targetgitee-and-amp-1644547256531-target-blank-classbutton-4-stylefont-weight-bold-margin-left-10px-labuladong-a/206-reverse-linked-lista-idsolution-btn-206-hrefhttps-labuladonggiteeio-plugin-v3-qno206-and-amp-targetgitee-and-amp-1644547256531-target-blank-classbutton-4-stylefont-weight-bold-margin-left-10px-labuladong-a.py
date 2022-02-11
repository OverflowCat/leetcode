# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None:
            return None
        if head.next is None:
            return head
        li = []
        node = head
        while node.next:
            li.append(node.val)
            node = node.next
        li.append(node.val)
        
        lengthd = len(li) - 1
        nohead = ListNode()
        node = nohead
        for x in reversed(li):
            node.next = ListNode(x)
            node = node.next
        return nohead.next #head