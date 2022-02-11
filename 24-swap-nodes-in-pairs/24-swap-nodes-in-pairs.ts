/**
 * Definition for singly-linked list.
 * class ListNode {
 *     val: number
 *     next: ListNode | null
 *     constructor(val?: number, next?: ListNode | null) {
 *         this.val = (val===undefined ? 0 : val)
 *         this.next = (next===undefined ? null : next)
 *     }
 * }
 */

function swapPairs(首: ListNode | null): ListNode | null {
    if (首 == null) return null
    let 奇 = 首
    if (奇.next == null) return 奇
    首 = 奇.next
    while (true) {
        let 偶 = 奇.next
        if (偶.next == null) {
            偶.next = 奇
            奇.next = null
            return 首
        }
        let 三 = 偶.next
        偶.next = 奇
        if (三.next == null) {
            奇.next = 三
            return 首
        } else {
            奇.next = 三.next
        }
        奇 = 三
    }
}