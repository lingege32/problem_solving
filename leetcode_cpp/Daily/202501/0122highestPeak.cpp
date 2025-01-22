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
    static vector<vector<int>> highestPeak(vector<vector<int>>& isWater) {
        int row = isWater.size(), col = isWater[0].size();
        std::vector<std::pair<int, int>> one,two;
        one.reserve(row*col);
        two.reserve(row*col);
        std::vector<std::pair<int, int>> *cur = &one, *next = &two;
        constexpr std::array<int, 4> dx = {0, 0, 1, -1};
        constexpr std::array<int, 4> dy = {1, -1, 0, 0};
        for (int r = 0; r < row; ++r) {
            for (int c = 0; c < col; ++c) {
                if (std::exchange(isWater[r][c], -1) == 1) {
                    cur->emplace_back(r, c);
                }
            }
        }
        int level = 0;
        while (!cur->empty()) {
            next->clear();
            for (auto [r, c] : *cur) {
                if (isWater[r][c] != -1) {
                    continue;
                }
                isWater[r][c] = level;
                for (int i = 0; i < 4; ++i) {
                    int nr = r + dx[i], nc = c + dy[i];
                    if (nr >= 0 && nr < row && nc >= 0 && nc < col && isWater[nr][nc] == -1) {
                        next->emplace_back(nr, nc);
                    }
                }
            }
            one.swap(two);
            level++;
        }
        return std::move(isWater);
    }
};