# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        if list1 is None:
            return list2
        if list2 is None:
            return list1
        cur1 = list1
        cur2 = list2
        head = ListNode()
        cur = head
        while cur1 is not None or cur2 is not None:
            if cur1 is None:
                cur.next = cur2
                break
            if cur2 is None:
                cur.next = cur1
                break
            if cur1.val == cur2.val:
                cur.next = ListNode(cur1.val)
                cur = cur.next
                cur.next = ListNode(cur2.val)
                cur1 = cur1.next
                cur2 = cur2.next
            elif cur1.val < cur2.val:
                cur.next = ListNode(cur1.val)
                cur1 = cur1.next
            else: # elif cur2.val < cur1.val:
                cur.next = ListNode(cur2.val)
                cur2 = cur2.next
            cur = cur.next
        return head.next