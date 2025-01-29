#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    int findRoot(std::vector<int>& root, int i) {
        if (root[i] != i) {
            root[i] = findRoot(root, root[i]);
        }
        return root[i];
    }

    vector<int> findRedundantConnection(vector<vector<int>>& edges) {
        int n = edges.size();
        std::vector<int> root(n + 1);
        for (int i = 1; i <= n; i++) {
            root[i] = i;
        }

        for (auto& edge : edges) {
            auto u = edge[0];
            auto v = edge[1];
            auto ru = findRoot(root, u);
            auto rv = findRoot(root, v);
            if (ru == rv) {
                return edge;
            }
            root[ru] = rv;
        }
        std::unreachable();
    }
};
