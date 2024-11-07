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
    static vector<int> treeQueries(TreeNode *root, vector<int> &queries) {
        std::unordered_map<int, std::pair<TreeNode *, int>> parents;

        buildParent(parents, root, nullptr);
        std::vector<int> ret;
        ret.reserve(queries.size());
        int top = parents[root->val].second;
        for (auto query : queries) {
            ret.push_back(std::min(top, solve(parents, query)));
        }
        return ret;
    }

    static int solve(const std::unordered_map<int, std::pair<TreeNode *, int>> &parents, int removed_node) {
        auto &pair = parents.at(removed_node);
        auto cur = pair.first;
        int level = -1;
        while (cur) {
            int another_level = -1;
            if (cur->left && cur->left->val == removed_node) {
                if (cur->right) {
                    another_level = parents.at(cur->right->val).second;
                }
            } else {
                if (cur->left) {
                    another_level = parents.at(cur->left->val).second;
                }
            }
            level = 1 + std::max(level, another_level);
            if (level == parents.at(cur->val).second) {
                return std::numeric_limits<int>::max();
            }
            removed_node = cur->val;
            cur = parents.at(cur->val).first;
        }
        return level;
    }

    static int buildParent(std::unordered_map<int, std::pair<TreeNode *, int>> &parents, TreeNode *root,
                           TreeNode *parent) {
        if (!root) {
            return -1;
        }

        auto left = buildParent(parents, root->left, root);
        auto right = buildParent(parents, root->right, root);
        auto lv = 1 + std::max(left, right);
        parents[root->val] = std::make_pair(parent, lv);
        return lv;
    }
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
class OptSolution {
  public:
    int lvl[100001];
    int firstmax[100001] = {0};
    int secondmax[100001] = {0};
    int height[100001];

    int solve(TreeNode *root, int level) {
        if (!root) {
            return 0;
        }

        height[root->val] = max(solve(root->left, level + 1), solve(root->right, level + 1)) + 1;
        lvl[root->val] = level;

        if (height[root->val] > firstmax[level]) {
            secondmax[level] = firstmax[level];
            firstmax[level] = height[root->val];
        } else if (height[root->val] > secondmax[level]) {
            secondmax[level] = height[root->val];
        }

        return height[root->val];
    }

    vector<int> treeQueries(TreeNode *root, vector<int> &queries) {
        solve(root, 0);
        vector<int> ans;
        for (int querie : queries) {
            int level = lvl[querie];
            if (firstmax[level] == height[querie]) {
                ans.push_back(level + secondmax[level] - 1);
            } else {
                ans.push_back(level + firstmax[level] - 1);
            }
        }
        return ans;
    }
};