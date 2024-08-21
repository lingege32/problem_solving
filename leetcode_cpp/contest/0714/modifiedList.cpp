
#include <bits/stdc++.h>
using namespace std;
struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class Solution {
  public:
    ListNode *modifiedList(vector<int> &nums, ListNode *head) {
        std::unordered_set<int> s(nums.begin(), nums.end());
        ListNode dummy{0, head};
        ListNode *cur = &dummy;
        while (cur && cur->next) {
            auto it = s.find(cur->next->val);
            if (it == s.end()) {
                cur = cur->next;
            } else {
                cur->next = cur->next->next;
            }
        }
        return dummy.next;
    }
};