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
    static long long kthLargestLevelSum(TreeNode *root, int k) {
        std::vector<long long> levelValues;

        std::vector<TreeNode *> oneLevel;
        oneLevel.push_back(root);

        while (!oneLevel.empty()) {
            std::vector<TreeNode *> tmp;
            long long v = 0;
            for (auto *node : oneLevel) {
                v += node->val;
                if (node->left) {
                    tmp.push_back(node->left);
                }
                if (node->right) {
                    tmp.push_back(node->right);
                }
            }
            tmp.swap(oneLevel);
            levelValues.push_back(v);
        }
        if (static_cast<size_t>(k) > levelValues.size()) {
            return -1;
        }
        std::nth_element(levelValues.begin(), levelValues.begin() + k - 1, levelValues.end(), std::greater<>());
        return levelValues[k - 1];
    }
};