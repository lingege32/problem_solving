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
    static int minimumTime(vector<vector<int>>& grid) {
        if (grid[0][1] > 1 && grid[1][0] > 1) {
            return -1;
        }
        int row = grid.size();
        int col = grid[0].size();
        static constexpr std::array<std::pair<int, int>, 4> DIR = {{{0, 1}, {0, -1}, {1, 0}, {-1, 0}}};
        using ThreeD = std::tuple<int, int, int>;
        std::priority_queue<ThreeD, std::vector<ThreeD>, std::greater<>> pq;
        std::vector<int> d(row * col, std::numeric_limits<int>::max());
        d[0] = 0;
        pq.emplace(0, 0, 0);
        while (!pq.empty()) {
            auto [t, x, y] = pq.top();
            pq.pop();
            if (x + 1 == row && y + 1 == col) {
                return t;
            }
            for (auto [dx, dy] : DIR) {
                auto nx = x + dx;
                auto ny = y + dy;
                if (nx >= 0 && nx < row && ny >= 0 && ny < col) {
                    int w = (grid[nx][ny] - t) % 2 == 0 ? 1 : 0;
                    int next = std::max(t + 1, grid[nx][ny] + w);
                    if (next < d[(nx * col) + ny]) {
                        d[(nx * col) + ny] = next;
                        pq.emplace(next, nx, ny);
                    }
                }
            }
        }

        return -1;
    }
};
