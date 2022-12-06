class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if not (head and head.next): return head
        odd, even, althead = head, head.next, head.next
        while even:
            if odd.next:
                odd.next = odd.next.next
                if odd.next: odd = odd.next
                else: break
            else:
                break
            if even.next:
                even.next = even.next.next
                even = even.next
            else:
                break
        odd.next = althead
        return head