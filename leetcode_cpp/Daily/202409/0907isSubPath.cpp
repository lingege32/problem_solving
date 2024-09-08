
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

struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;

    TreeNode():
      val(0),
      left(nullptr),
      right(nullptr) {}

    explicit TreeNode(int x):
      val(x),
      left(nullptr),
      right(nullptr) {}

    TreeNode(int x, TreeNode *left, TreeNode *right):
      val(x),
      left(left),
      right(right) {}
};

#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static bool isSubPath(ListNode *head, TreeNode *root) {
        if (head == nullptr) {
            return true;
        }
        if (root == nullptr) {
            return false;
        }
        if (root->val == head->val && checkSequentialList(head, root)) {
            return true;
        }
        return isSubPath(head, root->left) || isSubPath(head, root->right);
    }

    static bool checkSequentialList(ListNode *head, TreeNode *root) {
        if (head == nullptr) {
            return true;
        }
        if (root == nullptr) {
            return false;
        }

        if (root->val == head->val) {
            return checkSequentialList(head->next, root->left) || checkSequentialList(head->next, root->right);
        }
        return false;
    }
};