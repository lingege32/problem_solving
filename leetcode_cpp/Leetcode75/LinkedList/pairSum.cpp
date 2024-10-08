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
    static int pairSum(ListNode* head) {
        auto n = count(head);
        n /= 2;
        auto tail = head;
        for (int i = 0; i < n; ++i) {
            tail = tail->next;
        }
        tail = reverse(tail);
        int max = 0;
        while (head && tail) {
            max = std::max(max, head->val + tail->val);
            head = head->next;
            tail = tail->next;
        }
        return max;
    }

    static int count(ListNode* head) {
        int n = 0;
        while (head) {
            n++;
            head = head->next;
        }
        return n;
    }

    static ListNode* reverse(ListNode* head) {
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