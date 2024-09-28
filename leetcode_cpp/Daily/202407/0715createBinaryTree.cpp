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

static std::array<TreeNode, 10001> nodes;

class Solution {
  public:
    static TreeNode *createBinaryTree(vector<vector<int>> &descriptions) {
        nodes.fill(TreeNode());
        int node_idx = 0;
        std::array<TreeNode *, 100001> mapping;
        mapping.fill(nullptr);

        auto getOrCreate = [&](int val) {
            auto &node = mapping[val];
            if (!node) {
                node = &nodes[node_idx++];
                node->val = -val;
            }
            return node;
        };
        for (auto &des : descriptions) {
            auto p = des[0];
            auto c = des[1];
            bool isLeft = des[2] == 1;
            auto parent = getOrCreate(p);
            auto child = getOrCreate(c);
            child->val = std::abs(child->val);
            if (isLeft) {
                parent->left = child;
            } else {
                parent->right = child;
            }
        }
        for (int idx = 0; idx < node_idx; ++idx) {
            auto &node = nodes[idx];
            if (node.val < 0) {
                node.val = -node.val;
                return &node;
            }
        }
        return nullptr;
    }
};