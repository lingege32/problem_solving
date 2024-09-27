#include <bits/stdc++.h>

#include <algorithm>
#include <ranges>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

/**
 * Definition for a binary tree node.
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
    int d;
    int cnt = 0;

  public:
    int countPairs(TreeNode *root, int distance) {
        d = distance - 2;
        count(root);
        return cnt;
    }

    std::array<int, 9> count(TreeNode *root) {
        if (!root) {
            return {0};
        }
        if (!root->left && !root->right) {
            return {1, 0};
        }
        auto lhs = count(root->left);
        auto rhs = count(root->right);
        for (int i = 0; i <= d; ++i) {
            for (int j = 0; j <= d - i; ++j) {
                cnt += lhs[i] * rhs[j];
            }
        }
        std::ranges::rotate(std::ranges::reverse_view(lhs), lhs.rbegin() + 1);
        lhs[0] = 0;
        for (int i = 1; i < 9; ++i) {
            lhs[i] += rhs[i - 1];
        }
        return lhs;
    }
};