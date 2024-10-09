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

class OptimizeSolution {
  public:
    void pathSumRec(TreeNode *root, int target, long sum, unordered_map<long, int> &hm, int &res) {
        if (!root) {
            return;
        }
        sum += root->val;
        if (sum == target) {
            res++;
        }
        if (hm.find(sum - target) != hm.end()) {
            res += hm[sum - target];
        }
        hm[sum]++;
        pathSumRec(root->left, target, sum, hm, res);
        pathSumRec(root->right, target, sum, hm, res);
        hm[sum]--;
    }

    int pathSum(TreeNode *root, int targetSum) {
        unordered_map<long, int> hm;
        int res = 0;
        long sum = 0;
        pathSumRec(root, targetSum, sum, hm, res);
        return res;
    }
};

class Solution {
  public:
    int pathSum(TreeNode *root, int targetSum) {
        if (!root) {
            return 0;
        }
        return inner(root, targetSum) + pathSum(root->left, targetSum) + pathSum(root->right, targetSum);
    }

    int inner(TreeNode *root, int targetSum) {
        if (!root) {
            return 0;
        }
        int ans = 0;
        if (root->val == targetSum) {
            ans++;
        }
        targetSum -= root->val;
        ans += inner(root->left, targetSum);
        ans += inner(root->right, targetSum);
        return ans;
    }
};