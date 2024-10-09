#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

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
    int ans = 0;

  public:
    int longestZigZag(TreeNode *root) {
        if (!root) {
            return 0;
        }
        inner(root->left, 1, false);
        inner(root->right, 1, true);
        return ans;
    }

    void inner(TreeNode *root, int depth, bool dir) {
        if (!root) {
            return;
        }
        ans = std::max(ans, depth);
        if (dir) {
            inner(root->left, depth + 1, false);
            inner(root->right, 1, true);
        } else {
            inner(root->left, 1, false);
            inner(root->right, depth + 1, true);
        }
    }
};