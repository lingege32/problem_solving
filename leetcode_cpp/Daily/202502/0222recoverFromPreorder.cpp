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

class Solution {
  public:
    static void handle(std::string &traversal, int n) {
        std::ranges::rotate(traversal, traversal.begin() + n);
        traversal.resize(traversal.size() - n);
    }

    static TreeNode *recoverFromPreorder(string traversal) {
        std::vector<TreeNode *> stack;
        auto dash = traversal.find('-');
        if (dash == std::string::npos) {
            return new TreeNode(std::stoi(traversal));
        }
        stack.push_back(new TreeNode(std::stoi(traversal.substr(0, dash))));
        auto next = traversal.find_first_not_of('-', dash);
        auto level = next - dash;
        handle(traversal, next);
        for (;;) {
            if (stack.size() > level) {
                stack.resize(level);
            }

            dash = std::min(traversal.size(), traversal.find('-'));
            auto val = std::stoi(traversal.substr(0, dash));
            auto *new_node = new TreeNode(val);

            auto *b = stack.back();
            if (b->left) {
                b->right = new_node;
            } else {
                b->left = new_node;
            }
            next = traversal.find_first_not_of('-', dash);
            if (next == std::string::npos) {
                break;
            }

            stack.push_back(new_node);
            level = next - dash;
            handle(traversal, next);
        }

        return stack.front();
    }
};

class OptSolution {
  public:
    string s;
    size_t idx = 0, level = 0;

    TreeNode *recoverFromPreorder(string traversal) {
        s = std::move(traversal);
        auto *node = new TreeNode(-1);
        helper(node, 0);
        return node->left;
    }

    void helper(TreeNode *parent, size_t lvl) {
        while (idx < s.length() && lvl == level) {
            int num = 0;
            while (idx < s.length() && isdigit(s[idx])) {
                num = num * 10 + (s[idx++] - '0');
            }
            auto *node = new TreeNode(num);
            if (!parent->left) {
                parent->left = node;
            } else {
                parent->right = node;
            }
            level = 0;
            while (idx < s.length() && s[idx] == '-') {
                level++;
                idx++;
            }
            helper(node, lvl + 1);
        }
    }
};