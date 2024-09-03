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
    vector<vector<int>> construct2DArray(vector<int>& original, int m, int n) {
        if (static_cast<size_t>(m * n) != original.size()) {
            return vector<vector<int>>{};
        }
        vector<vector<int>> ans(m, vector<int>());
        auto be = original.begin();
        for (int i = 0; i < m; ++i, be += n) {
            auto& row = ans[i];
            row = std::vector<int>{be, be + n};
        }
        return ans;
    }
};