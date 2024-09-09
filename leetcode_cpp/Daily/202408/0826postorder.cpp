
#include <bits/stdc++.h>
using namespace std;

class Node {
  public:
    int val;
    vector<Node*> children;

    Node() = default;

    explicit Node(int _val) { val = _val; }

    Node(int _val, vector<Node*> _children):
      val(_val),
      children(std::move(_children)) {}
};

class Solution {
    void access(Node* r, vector<int>& res) {
        if (!r) {
            return;
        }
        for (auto c : r->children) {
            access(c, res);
        }
        res.push_back(r->val);
    }

  public:
    vector<int> postorder(Node* root) {
        vector<int> ans;
        access(root, ans);
        return ans;
    }
};