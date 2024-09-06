#include <bits/stdc++.h>
using namespace std;

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
    static ListNode* modifiedList(vector<int>& nums, ListNode* head) {
        // unordered_set<int> s{nums.begin(), nums.end()};
        std::sort(nums.begin(), nums.end());
        ListNode dummy;
        ListNode** cur = &dummy.next;
        while (head) {
            auto next = head->next;
            if (!binary_search(nums.begin(),nums.end(),head->val)) {
                head->next=nullptr;
                *cur = head;
                cur = &(*cur)->next;
            }
            head = next;
        }
        return dummy.next;
    }
};