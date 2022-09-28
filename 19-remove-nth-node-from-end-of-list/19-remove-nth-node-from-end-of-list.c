/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */


struct ListNode* removeNthFromEnd(struct ListNode* head, int n){
    struct ListNode* curr = head;
    struct ListNode* t[30];
    unsigned short i = 0;
    while(curr) {
        t[i++] = curr;
        curr = curr->next;
    }
    if (n == 1) {
        if (i == 1) return NULL;
        t[i - 2]->next = NULL;
        return head;
    }
    unsigned short j = i - n;
    if (j == 0) return head->next;
    t[j - 1] -> next = t[j + 1];
    return head;
}