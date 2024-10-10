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
    static int maxLevelSum(TreeNode *root) {
        if (!root) {
            return 0;
        }
        std::vector<TreeNode *> v{root};
        std::vector<TreeNode *> tmp;
        int max = std::numeric_limits<int>::min();
        int min_level = 0;
        int level = 1;
        while (!v.empty()) {
            auto sum = std::accumulate(v.begin(), v.end(), 0, [](int sum, TreeNode *node) { return sum + node->val; });
            if (max < sum) {
                max = sum;
                min_level = level;
            }
            for (auto node : v) {
                if (node->left) {
                    tmp.push_back(node->left);
                }
                if (node->right) {
                    tmp.push_back(node->right);
                }
            }

            level++;
            v.swap(tmp);
            tmp.clear();
        }
        return min_level;
    }
};