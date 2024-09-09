#include <bits/stdc++.h>
using namespace std;

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

class Solution {
    void access(vector<int> &ans, TreeNode *r) {
        if (!r) {
            return;
        }
        access(ans, r->left);
        access(ans, r->right);
        ans.push_back(r->val);
    }

  public:
    vector<int> postorderTraversal(TreeNode *root) {
        std::vector<int> ans;
        access(ans, root);
        return ans;
    }
};