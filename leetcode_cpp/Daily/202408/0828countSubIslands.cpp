#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static bool access(vector<vector<int>>& grid, const vector<vector<int>>& base, int x, int y) {
        if (x < 0 || y < 0 || static_cast<size_t>(x) >= grid.size() || static_cast<size_t>(y) >= grid[0].size()) {
            return true;
        }
        auto& g = grid[x][y];
        if (g == 0) {
            return true;
        }
        g=0;
        bool res = base[x][y] == 1;
        auto l1 = access(grid, base, x - 1, y);
        auto l2 = access(grid, base, x + 1, y);
        auto l3 = access(grid, base, x, y - 1);
        auto l4 = access(grid, base, x, y + 1);
        return res && l1 && l2 && l3 && l4;
    }

    static int countSubIslands(vector<vector<int>>& grid1, vector<vector<int>>& grid2) {
        int ans = 0;
        for (size_t x = 0; x < grid2.size(); ++x) {
            for (size_t y = 0; y < grid2[0].size(); ++y) {
                if (grid2[x][y] == 1 && access(grid2, grid1, x, y)) {
                    ans++;
                }
            }
        }
        return ans;
    }
};