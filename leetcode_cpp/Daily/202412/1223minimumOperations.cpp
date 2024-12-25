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
    static int dfs(const std::vector<int> &indexed, int idx, std::vector<bool> &visited) {
        if (visited[idx]) {
            return 0;
        }
        visited[idx] = true;
        return dfs(indexed, indexed[idx], visited) + 1;
    }

    static int calSwap(const std::vector<TreeNode *> &level) {
        std::vector<int> indexed;
        int n = level.size();
        indexed.reserve(n);
        for (int i = 0; i < n; ++i) {
            indexed.emplace_back(i);
        }
        std::ranges::sort(indexed, [&](int a, int b) { return level[a]->val < level[b]->val; });
        std::vector<bool> visited(n, false);
        int ret = 0;
        for (int i = 0; i < n; ++i) {
            if (!visited[i]) {
                ret += dfs(indexed, i, visited) - 1;
            }
        }
        return ret;
    }

    static int minimumOperations(TreeNode *root) {
        int n = 0;
        std::vector<TreeNode *> level;
        level.push_back(root);
        while (!level.empty()) {
            std::vector<TreeNode *> next_level;
            for (auto *node : level) {
                if (node->left) {
                    next_level.push_back(node->left);
                }
                if (node->right) {
                    next_level.push_back(node->right);
                }
            }
            n += calSwap(level);
            std::swap(level, next_level);
        }
        return n;
    }
};