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
    TreeNode* deleteNode(TreeNode* root, int key) {
        if (root) {
            if (key < root->val) {
                root->left = deleteNode(root->left, key);  // We frecursively call the function until we find the target
                                                           // node
            } else if (key > root->val) {
                root->right = deleteNode(root->right, key);
            } else {
                if (!root->left && !root->right) {
                    return nullptr;  // No child condition
                }
                if (!root->left) {
                    return root->right;
                }
                if (!root->right) {
                    return root->left;
                }
                TreeNode* temp = root->left;  //(or) TreeNode *temp = root->right;
                while (temp->right != nullptr) {
                    temp = temp->right;  //      while(temp->left != NULL) temp = temp->left;
                }
                root->val = temp->val;                           //       root->val = temp->val;
                root->left = deleteNode(root->left, temp->val);  //        root->right = deleteNode(root->right, temp);
            }
        }
        return root;
    }
};