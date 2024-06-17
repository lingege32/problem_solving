#include <bits/stdc++.h>
using namespace std;
class Solution {
  public:
    int maxScore(vector<vector<int>> &grid) {
        int ret = std::numeric_limits<int>::min();
        auto row = grid.size();
        auto col = grid[0].size();
        for (int idx = col - 2; idx >= 0; --idx) {
            auto val = grid.back()[idx];
            auto right_max = grid.back()[idx + 1];
            ret = std::max(ret, right_max - val);
            grid.back()[idx] = std::max(val, right_max);
        }
        for (int idx = row - 2; idx >= 0; --idx) {
            auto val = grid[idx].back();
            auto down_max = grid[idx + 1].back();
            ret = std::max(ret, down_max - val);
            grid[idx].back() = std::max(val, down_max);
        }
        for (int row_idx = row - 2; row_idx >= 0; --row_idx) {
            for (int col_idx = col - 2; col_idx >= 0; --col_idx) {
                auto val = grid[row_idx][col_idx];
                auto m = std::max(grid[row_idx + 1][col_idx],
                                  grid[row_idx][col_idx + 1]);
                ret = std::max(ret, m - val);
                grid[row_idx][col_idx] = std::max(val, m);
            }
        }
        return ret;
    }
};