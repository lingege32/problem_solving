#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    int minimumArea(vector<vector<int>> &grid) {
        auto row = grid.size();
        auto col = grid[0].size();
        size_t left = col, right = 0;
        size_t up = 0, down = row;
        for (size_t r = 0U; r < row; ++r) {
            for (size_t c = 0U; c < col; ++c) {
                if (grid[r][c] == 1) {
                    left = std::min(left, c);
                    right = std::max(right, c);
                    up = std::max(up, r);
                    down = std::min(down, r);
                }
            }
        }
        return (up - down + 1) * (right - left + 1);
    }
};