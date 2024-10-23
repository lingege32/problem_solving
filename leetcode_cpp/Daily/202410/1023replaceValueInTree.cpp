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
  public:
    static TreeNode *replaceValueInTree(TreeNode *root) {
        std::vector<std::pair<int, TreeNode *>> level;
        level.emplace_back(root->val, root);
        while (!level.empty()) {
            std::vector<std::pair<int, TreeNode *>> nextLevel;
            int sum = 0;
            for (auto &[val, node] : level) {
                sum += node->val;
                int childva = 0;
                if (node->left) {
                    childva += node->left->val;
                }
                if (node->right) {
                    childva += node->right->val;
                    nextLevel.emplace_back(childva, node->right);
                }
                if (node->left) {
                    nextLevel.emplace_back(childva, node->left);
                }
            }
            for (auto &[val, node] : level) {
                node->val = sum - val;
            }
            nextLevel.swap(level);
        }
        return root;
    }
};

auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
