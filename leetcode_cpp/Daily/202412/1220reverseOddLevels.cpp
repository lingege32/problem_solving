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
  public:
    static TreeNode *reverseOddLevels(TreeNode *root) {
        if (!root) {
            return nullptr;
        }
        std::vector<TreeNode *> nodes;
        nodes.push_back(root);
        int i = 0;
        for (;;) {
            auto curLevel = i++;
            if (curLevel % 2 == 1) {
                int l = 0, r = nodes.size() - 1;
                while (l < r) {
                    std::swap(nodes[l++]->val, nodes[r--]->val);
                }
            }
            if (nodes[0]->left == nullptr) {
                break;
            }
            std::vector<TreeNode *> nextLevel;
            nextLevel.reserve(nodes.size() * 2);
            for (auto node : nodes) {
                nextLevel.push_back(node->left);
                nextLevel.push_back(node->right);
            }
            std::swap(nextLevel, nodes);
        }
        return root;
    }
};