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

class FindElements {
    static inline std::array<uint8_t, 1000001> mExist;

    void inner(TreeNode *node, int cur) {
        if (!node || static_cast<size_t>(cur) > mExist.size()) {
            return;
        }
        mExist[cur] = 1;
        inner(node->left, (2 * cur) + 1);
        inner(node->right, (2 * cur) + 2);
    }

  public:
    explicit FindElements(TreeNode *root) {
        mExist.fill(0);
        inner(root, 0);
    }

    ~FindElements() { mExist.fill(0); }

    static bool find(int target) { return mExist[target]; }
};

/**
 * Your FindElements object will be instantiated and called as such:
 * FindElements* obj = new FindElements(root);
 * bool param_1 = obj->find(target);
 */