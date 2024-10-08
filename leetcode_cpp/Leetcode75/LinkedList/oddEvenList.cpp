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
    ListNode *next;

    ListNode():
      val(0),
      next(nullptr) {}

    explicit ListNode(int x):
      val(x),
      next(nullptr) {}

    ListNode(int x, ListNode *next):
      val(x),
      next(next) {}
};

class Solution {
  public:
    static ListNode *oddEvenList(ListNode *head) {
        ListNode d1, d2;
        ListNode **odd = &d1.next;
        ListNode **even = &d2.next;
        bool isOdd = true;
        while (head) {
            ListNode *next = nullptr;
            std::swap(next, head->next);
            if (isOdd) {
                *odd = head;
                odd = &(*odd)->next;
            } else {
                *even = head;
                even = &(*even)->next;
            }
            head = next;
            isOdd = !isOdd;
        }
        *odd = d2.next;
        return d1.next;
    }
};