#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct TwoDArray {
    std::vector<uint8_t> data;
    int row = 0;

    TwoDArray(int m, int n):
      data(m * n, 0),
      row{n} {}

    uint8_t* operator[](int i) { return &data[i * row]; }
};

constexpr char EMPTY = 0;
constexpr char KILLED = 1;
constexpr char GUARD = 2;
constexpr char WALL = 3;

class Solution {
  public:
    static int handleCell(uint8_t& cell, int& ret) {
        if (cell == EMPTY) {
            cell = KILLED;
            ret--;
            return 0;
        }
        if (cell == KILLED) {
            return 1;
        }
        return 2;
    }

    static void handleGuard(const auto& guard, TwoDArray& grid, int m, int n, int& ret) {
        int x = guard[0], y = guard[1];
        for (int i = x + 1; i < m; ++i) {
            auto& cell = grid[i][y];
            auto r = handleCell(cell, ret);
            if (r == 1) {
                continue;
            }
            if (r == 2) {
                break;
            }
        }
        for (int i = x - 1; i >= 0; --i) {
            auto& cell = grid[i][y];
            auto r = handleCell(cell, ret);
            if (r == 1) {
                continue;
            }
            if (r == 2) {
                break;
            }
        }
        for (int i = y + 1; i < n; ++i) {
            auto& cell = grid[x][i];
            auto r = handleCell(cell, ret);
            if (r == 1) {
                continue;
            }
            if (r == 2) {
                break;
            }
        }
        for (int i = y - 1; i >= 0; --i) {
            auto& cell = grid[x][i];
            auto r = handleCell(cell, ret);
            if (r == 1) {
                continue;
            }
            if (r == 2) {
                break;
            }
        }
    }

    static int countUnguarded(int m, int n, vector<vector<int>>& guards, vector<vector<int>>& walls) {
        TwoDArray grid(m, n);
        int ret = m * n;
        for (const auto& guard : guards) {
            grid[guard[0]][guard[1]] = GUARD;
        }
        for (const auto& wall : walls) {
            grid[wall[0]][wall[1]] = WALL;
        }
        ret -= walls.size();
        ret -= guards.size();
        for (auto& guard : guards) {
            handleGuard(guard, grid, m, n, ret);
        }
        return ret;
    };
};