# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def isPalindrome(self, head: Optional[ListNode]) -> bool:
        curr = head
        vec = [head.val]
        while curr.next is not None:
            curr = curr.next
            vec.append(curr.val)
        # print("vec is", vec)
        i = 0
        j = len(vec)
        while i < j:
            j -= 1
            if vec[i] != vec[j]:
                return False
            i += 1
        return True