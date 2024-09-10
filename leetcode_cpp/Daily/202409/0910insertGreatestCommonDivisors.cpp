#include <bits/stdc++.h>
using namespace std;

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
    static ListNode* insertGreatestCommonDivisors(ListNode* head) {
        auto prev = head;
        auto cur = head->next;
        while (cur) {
            auto gcd = std::gcd(cur->val, prev->val);
            auto node = new ListNode(gcd, cur);
            prev->next = node;
            prev = cur;
            cur = cur->next;
        }
        return head;
    }
};