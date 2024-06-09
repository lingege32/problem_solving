// Definition for a binary tree node.
struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right)
        : val(x), left(left), right(right) {}
};
class Solution {
  public:
    // Don't delete the root
    TreeNode *removeLeafNodes(TreeNode *root, int target) {
        return helper(root, target, true);
    }

    TreeNode *helper(TreeNode *root, int target, bool isRoot) {
        if (!root) {
            return root;
        }

        root->left = helper(root->left, target, false);
        root->right = helper(root->right, target, false);
        if (root->left || root->right) {
            return root;
        }
        if (root->val == target) {
            if (!isRoot) {

                delete root;
            }
            return nullptr;
        }
        return root;
    }
};