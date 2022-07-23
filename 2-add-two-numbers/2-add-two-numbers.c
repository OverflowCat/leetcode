/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     struct ListNode *next;
 * };
 */

typedef struct ListNode listNode;

struct ListNode *addTwoNumbers(struct ListNode *l1, struct ListNode *l2)
{
    listNode *curr1 = l1;
    listNode *curr2 = l2;
    listNode *curr3 = (listNode *)malloc(sizeof(listNode));
    listNode *guard3 = curr3;
    int flag = 0;
    while (curr1 != NULL || curr2 != NULL)
    {
        listNode *new = (listNode *)malloc(sizeof(listNode));
        curr3->next = new;
        curr3 = new;
        unsigned sum = flag;
        if (curr1 != NULL)
        {
            sum += curr1->val;
            curr1 = curr1->next;
        }
        if (curr2 != NULL)
        {
            sum += curr2->val;
            curr2 = curr2->next;
        }
        if (sum > 9)
        {
            flag = 1;
            sum -= 10;
        }
        else
        {
            flag = 0;
        }
        curr3->val = sum;
        curr3->next = NULL;
    }
    if (flag)
    {
        listNode *new = (listNode *)malloc(sizeof(listNode));
        curr3->next = new;
        curr3 = new;
        curr3->val = 1;
        curr3->next = NULL;
    }
    return guard3->next;
}
