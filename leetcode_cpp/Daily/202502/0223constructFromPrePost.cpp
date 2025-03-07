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

class Solution {
    static TreeNode *inner(auto preBegin, auto preEnd, auto postBegin, const auto& postEnd) {
        std::cout<<"\n";
        if (preBegin == preEnd) {
            return nullptr;
        }
        auto rootVal = *preBegin;
        auto *root = new TreeNode(rootVal);
        if (std::distance(preBegin, preEnd) == 1) {
            return root;
        }
        preBegin++;
        postBegin++;
        if (*preBegin == *postBegin) {
            root->left = inner(preBegin, preEnd, postBegin, postEnd);
            return root;
        }
        auto preRightIt = std::find(preBegin, preEnd, *postBegin);
        auto postLeftIt = std::find(postBegin, postEnd, *preBegin);
        root->left = inner(preBegin, preRightIt, postLeftIt, postEnd);
        root->right = inner(preRightIt, preEnd, postBegin, postLeftIt);
        return root;
    }

  public:
    static TreeNode *constructFromPrePost(vector<int> &preorder, vector<int> &postorder) {
        return inner(preorder.begin(), preorder.end(), postorder.rbegin(), postorder.rend());
    }
};