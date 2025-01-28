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
    static void dfs(int& fish, int x, int y, int row, int col, vector<vector<int>>& grid) {
        if (x < 0 || x >= row || y < 0 || y >= col || grid[x][y] == 0) {
            return;
        }
        fish += std::exchange(grid[x][y], 0);
        constexpr static std::array<int, 4> dx = {0, 0, 1, -1};
        constexpr static std::array<int, 4> dy = {1, -1, 0, 0};
        for (int i = 0; i < 4; ++i) {
            int nx = x + dx[i];
            int ny = y + dy[i];
            dfs(fish, nx, ny, row, col, grid);
        }
    }

    static int findMaxFish(vector<vector<int>>& grid) {
        int row = grid.size();
        int col = grid[0].size();
        int ret = 0;
        for (int r = 0; r < row; ++r) {
            for (int c = 0; c < col; ++c) {
                int fish = 0;
                dfs(fish, r, c, row, col, grid);
                ret = max(ret, fish);
            }
        }
        return ret;
    }
};