#include <bits/stdc++.h>
using namespace std;
struct Table {
    std::vector<std::pair<int, int>> m;
    size_t row;
    size_t col;
    Table(size_t row, size_t col)
        : m{row * col, std::make_pair(0, 0)}, row{row}, col{col} {}
    std::pair<int, int> &get(int r, int c) { return m[r * col + c]; }
};
class Solution {
  public:
    int numberOfSubmatrices(vector<vector<char>> &grid) {
        size_t row = grid.size();
        size_t col = grid[0].size();
        Table table{row + 1, col + 1};
        int ret = 0;
        for (size_t r = 1; r <= row; ++r) {
            for (size_t c = 1; c <= col; ++c) {
                auto &[x, y] = table.get(r, c);
                const auto &one = table.get(r - 1, c);
                const auto &two = table.get(r, c - 1);
                const auto &three = table.get(r - 1, c - 1);
                x = one.first + two.first - three.first;
                y = one.second + two.second - three.second;
                auto v = grid[r - 1][c - 1];
                if (v == 'X') {
                    x++;
                } else if (v == 'Y') {
                    y++;
                }
                if (x>0 && x == y) {
                    ret++;
                }
            }
        }
        return ret;
    }
};