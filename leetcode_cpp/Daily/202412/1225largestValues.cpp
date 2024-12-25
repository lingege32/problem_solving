#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

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
  public:
    static vector<int> largestValues(TreeNode *root) {
        std::vector<int> ret;
        if (!root) {
            return ret;
        }
        std::vector<TreeNode *> level;
        level.push_back(root);
        while (!level.empty()) {
            int v = std::numeric_limits<int>::min();
            std::vector<TreeNode *> next_level;
            for (auto node : level) {
                if (node->left) {
                    next_level.push_back(node->left);
                }
                if (node->right) {
                    next_level.push_back(node->right);
                }
                v = std::max(v, node->val);
            }
            ret.push_back(v);
            std::swap(level, next_level);
        }

        return ret;
    }
};