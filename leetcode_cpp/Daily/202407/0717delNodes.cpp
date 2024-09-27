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
    TreeNode* left;
    TreeNode* right;

    TreeNode():
      val(0),
      left(nullptr),
      right(nullptr) {}

    explicit TreeNode(int x):
      val(x),
      left(nullptr),
      right(nullptr) {}

    TreeNode(int x, TreeNode* left, TreeNode* right):
      val(x),
      left(left),
      right(right) {}
};

class Solution {
  public:
    static vector<TreeNode*> delNodes(TreeNode* root, vector<int>& to_delete) {
        std::unordered_set<int> to_delete_set(to_delete.begin(), to_delete.end());
        std::vector<TreeNode*> ans;
        auto r = inner(root, to_delete_set, ans);
        if (r) {
            ans.push_back(r);
        }
        return ans;
    }

    static TreeNode* inner(TreeNode* root, unordered_set<int>& to_delete_set, vector<TreeNode*>& ans) {
        if (!root) {
            return nullptr;
        }
        root->left = inner(root->left, to_delete_set, ans);
        root->right = inner(root->right, to_delete_set, ans);
        auto ext = to_delete_set.extract(root->val);
        if (!ext.empty()) {
            if (root->left) {
                ans.push_back(root->left);
            }
            if (root->right) {
                ans.push_back(root->right);
            }
            return nullptr;
        }
        return root;
    }
};

TreeNode* processNodes(TreeNode* node, unordered_set<int>& del, vector<TreeNode*>& forest) {
    if (node == nullptr) {
        return nullptr;
    }
    node->left = processNodes(node->left, del, forest);
    node->right = processNodes(node->right, del, forest);
    if (del.find(node->val) != del.end()) {
        if (node->left) {
            forest.push_back(node->left);
        }

        if (node->right) {
            forest.push_back(node->right);
        }

        return nullptr;
    }

    return node;
}

class OptimizeSolution {
  public:
    static vector<TreeNode*> delNodes(TreeNode* root, vector<int>& to_delete) {
        unordered_set<int> del(to_delete.begin(), to_delete.end());
        vector<TreeNode*> forest;

        TreeNode* node = processNodes(root, del, forest);
        if (node) {
            forest.push_back(node);
        }
        return forest;
    }
};

// find the nodes
//  o