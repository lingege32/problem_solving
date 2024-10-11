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
    static int orangesRotting(vector<vector<int>>& grid) {
        std::vector<std::pair<int, int>> stack;
        std::vector<std::pair<int, int>> tmp;
        int row = grid.size();
        int col = grid[0].size();
        int ones = 0;
        for (int i = 0; i < row; ++i) {
            for (int j = 0; j < col; ++j) {
                auto& g = grid[i][j];
                if (g == 1) {
                    ones++;
                }
                if (g == 2) {
                    g = 1;
                    stack.emplace_back(i, j);
                }
            }
        }
        int level = 0;
        if (stack.empty()) {
            return ones ? -1 : 0;
        }
        int mark = 1;
        while (!stack.empty()) {
            mark = 1;
            for (const auto& [x, y] : stack) {
                if (x < 0 || y < 0 || x == row || y == col) {
                    continue;
                }
                auto& m = grid[x][y];
                if (m != 1) {
                    continue;
                }
                mark = 0;
                m = 2;
                tmp.emplace_back(x + 1, y);
                tmp.emplace_back(x - 1, y);
                tmp.emplace_back(x, y + 1);
                tmp.emplace_back(x, y - 1);
            }
            level++;
            stack.swap(tmp);
            tmp.clear();
        }

        auto any_fresh = std::ranges::any_of(
            grid, [](const auto& row) { return std::ranges::any_of(row, [](const auto& cell) { return cell == 1; }); });
        return any_fresh ? -1 : level - 1 - mark;
    }
};