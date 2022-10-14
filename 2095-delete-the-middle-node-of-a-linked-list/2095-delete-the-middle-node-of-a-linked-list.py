class Solution:
    def deleteMiddle(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head.next is None: return None
        i, j = head, head.next
        while j.next:
            j = j.next
            if j.next is None: break
            i, j = i.next, j.next
        i.next = i.next.next
        return head