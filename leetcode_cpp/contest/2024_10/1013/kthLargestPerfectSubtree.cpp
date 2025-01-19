#include <bits/stdc++.h>
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

/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */

class Solution {
    std::vector<int> data;

  public:
    int kthLargestPerfectSubtree(TreeNode *root, int k) {
        inner(root);
        std::nth_element(data.begin(), data.begin() + k - 1, data.end(), std::greater<>());

        return static_cast<int>(data.size()) < k ? -1 : *(data.begin() + k - 1);
    }

    int inner(TreeNode *root) {
        if (!root) {
            return 0;
        }
        auto l = inner(root->left);
        auto r = inner(root->right);
        if (l == -1 || r == -1) {
            return -1;
        }
        if (l == r) {
            int v = (l * 2) + 1;
            data.push_back(v);
            return v;
        }
        return -1;
    }
};