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

class Solution {
  private:
    bool getPath(TreeNode *root, int value, string &path) {
        if (root == nullptr) {
            return false;
        }
        if (root->val == value) {
            return true;
        }
        path += 'L';
        if (getPath(root->left, value, path)) {
            return true;
        }
        path.pop_back();
        path += 'R';
        if (getPath(root->right, value, path)) {
            return true;
        }
        path.pop_back();
        return false;
    }

  public:
    string getDirections(TreeNode *root, int startValue, int destValue) {
        string a, b;
        getPath(root, startValue, a);
        getPath(root, destValue, b);
        int idx = 0;
        while (a[idx] == b[idx]) {
            a[idx] = '*';
            b[idx] = '*';
            idx++;
        }
        for (char &i : a) {
            if (i != '*') {
                i = 'U';
            }
        }
        a += b;
        string Res;
        for (char i : a) {
            if (i != '*') {
                Res += i;
            }
        }
        return Res;
    }
};