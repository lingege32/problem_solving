#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct TwoDArray {
    int col = 0;
    int row = 0;
    std::vector<int> data;

    TwoDArray(int m, int n):
      col{n},
      row{m},
      data(m * n, -1) {}

    int* operator[](int r) { return &data[r * col]; }

    int max() { return *std::ranges::max_element(data); }
};

class Solution {
  public:
    static int maxMoves(vector<vector<int>>& grid) {
        int m = grid.size();
        int n = grid[0].size();
        TwoDArray dp(m, n);
        for (int row = 0; row < m; ++row) {
            dp[row][0] = 0;
        }
        for (int col = 0; col < n - 1; ++col) {
            for (int row = 0; row < m; ++row) {
                auto d = dp[row][col];
                if (d == -1) {
                    continue;
                }
                d++;
                int v = grid[row][col];
                if (row > 0 && grid[row - 1][col + 1] > v) {
                    dp[row - 1][col + 1] = std::max(dp[row - 1][col + 1], d);
                }
                if (grid[row][col + 1] > v) {
                    dp[row][col + 1] = std::max(dp[row][col + 1], d);
                }
                if (row + 1 < m && grid[row + 1][col + 1] > v) {
                    dp[row + 1][col + 1] = std::max(dp[row + 1][col + 1], d);
                }
            }
        }
        return dp.max();
    }
};