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
    static void produce(TreeNode *root, std::vector<int> &num) {
        if (!root) {
            return;
        }
        if (!root->left && !root->right) {
            num.push_back(root->val);
        }
        produce(root->left, num);
        produce(root->right, num);
    }

    static bool consume(TreeNode *root, std::vector<int> &num) {
        if (!root) {
            return true;
        }
        if (!root->left && !root->right) {
            if (num.empty() || num.back() != root->val) {
                return false;
            }
            num.pop_back();
            return true;
        }
        if (!consume(root->right, num)) {
            return false;
        }
        if (!consume(root->left, num)) {
            return false;
        }
        return true;
    }

    static bool leafSimilar(TreeNode *root1, TreeNode *root2) {
        std::vector<int> num1;

        produce(root1, num1);
        auto s = num1.size();
        produce(root2, num1);
        if (s * 2 != num1.size()) {
            return false;
        }
        for (size_t idx = 0; idx < s; ++idx) {
            if (num1[idx] != num1[s + idx]) {
                return false;
            }
        }

        return true;
    }
};

class Solution2 {
  public:
    static void produce(TreeNode *root, std::vector<int> &num) {
        if (!root) {
            return;
        }
        if (!root->left && !root->right) {
            num.push_back(root->val);
        }
        produce(root->left, num);
        produce(root->right, num);
    }

    static bool consume(TreeNode *root, std::vector<int> &num) {
        if (!root->left && !root->right) {
            if (num.empty() || num.back() != root->val) {
                return false;
            }
            num.pop_back();
            return true;
        }
        if (root->right && !consume(root->right, num)) {
            return false;
        }
        if (root->left && !consume(root->left, num)) {
            return false;
        }
        return true;
    }

    static bool leafSimilar(TreeNode *root1, TreeNode *root2) {
        std::vector<int> num1;

        produce(root1, num1);
        auto ret = consume(root2, num1);
        return ret && num1.empty();
    }
};