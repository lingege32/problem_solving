#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
struct ListNode {
    int val;
    ListNode* next;

    ListNode():
      val(0),
      next(nullptr) {}

    explicit ListNode(int x):
      val(x),
      next(nullptr) {}

    ListNode(int x, ListNode* next):
      val(x),
      next(next) {}
};

class Solution {
  public:
    static ListNode* reverseList(ListNode* head) {
        ListNode dummy;
        while (head) {
            ListNode* next = head->next;
            head->next = dummy.next;
            dummy.next = head;
            head = next;
        }
        return dummy.next;
    }
};