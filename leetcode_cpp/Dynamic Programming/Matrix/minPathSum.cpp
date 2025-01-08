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
    static int minPathSum(vector<vector<int>>& grid) {
        int m = grid.size();
        int n = grid[0].size();
        for (int i = 1; i < n; ++i) {
            grid[0][i] += grid[0][i - 1];
        }

        for (int i = 1; i < m; ++i) {
            grid[i][0] += grid[i - 1][0];
        }
        for (int i = 1; i < m; ++i) {
            for (int j = 1; j < n; ++j) {
                auto min = std::min(grid[i - 1][j], grid[i][j - 1]);
                grid[i][j] += min;
            }
        }
        return grid.back().back();
    }
};