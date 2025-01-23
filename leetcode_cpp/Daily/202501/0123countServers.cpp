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
    static void handleRow(vector<vector<int>>& grid, std::vector<std::pair<int, int>>& servers, int row, int col) {
        for (int r = 0; r < row; ++r) {
            for (int c = 0; c < col; ++c) {
                if (grid[r][c] > 0) {
                    servers.emplace_back(r, c);
                }
            }
            if (servers.size() > 1) {
                for (auto [x, y] : servers) {
                    grid[x][y] = 2;
                }
            }
            servers.clear();
        }
    }

    static void handleCol(vector<vector<int>>& grid, std::vector<std::pair<int, int>>& servers, int row, int col) {
        for (int c = 0; c < col; ++c) {
            for (int r = 0; r < row; ++r) {
                if (grid[r][c] > 0) {
                    servers.emplace_back(r, c);
                }
            }
            if (servers.size() > 1) {
                for (auto [x, y] : servers) {
                    grid[x][y] = 2;
                }
            }
            servers.clear();
        }
    }

    static int countServers(vector<vector<int>>& grid) {
        int row = grid.size();
        int col = grid[0].size();
        std::vector<std::pair<int, int>> servers;
        handleRow(grid, servers, row, col);
        handleCol(grid, servers, row, col);

        int ret = 0;
        for (int r = 0; r < row; ++r) {
            for (int c = 0; c < col; ++c) {
                if (grid[r][c] ==2) {
                    ++ret;
                }
            }
        }
        return ret;
    }
};