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

#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static vector<ListNode *> splitListToParts(ListNode *head, int k) {
        std::vector<ListNode *> ans;
        ans.reserve(k);
        auto h = head;
        int total = 0;
        while (h) {
            ++total;
            h = h->next;
        }
        int s = total / k;
        int r = total % k;
        for (int i = 0; i < k; ++i) {
            ans.push_back(head);
            head = takeN(head, i < r ? s + 1 : s);
        }
        return ans;
    }

    static ListNode *takeN(ListNode *head, int n) {
        if (n == 0) {
            return head;
        }
        while (--n) {
            head = head->next;
        }
        auto nn = head->next;
        head->next = nullptr;
        return nn;
    }
};