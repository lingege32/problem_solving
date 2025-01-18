#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct TwoDArr {
    int col = 0;
    std::vector<int> data;

    TwoDArr(int row, int col):
      col(col),
      data(row * col, std::numeric_limits<int>::max()) {}

    int* operator[](int row) { return &data[row * col]; }
};

class Solution {
  public:
    static int minCost(vector<vector<int>>& grid) {
        int n = grid.size();
        int m = grid[0].size();

        constexpr std::array<int, 4> dx = {0, 0, 1, -1};
        constexpr std::array<int, 4> dy = {1, -1, 0, 0};

        TwoDArr dist(n, m);

        deque<pair<int, int>> dq;
        dq.emplace_back(0, 0);
        dist[0][0] = 0;

        while (dq.size() > 0) {
            auto curr = dq.front();
            dq.pop_front();
            int x = curr.first;
            int y = curr.second;
            int dir = grid[x][y];

            for (int i = 0; i < 4; i++) {
                int nx = x + dx[i];
                int ny = y + dy[i];

                int edgewt = (i + 1 != dir);
                if (nx == n || ny == m || nx < 0 || ny < 0 || dist[nx][ny] <= dist[x][y] + edgewt) {
                    continue;
                }
                dist[nx][ny] = dist[x][y] + edgewt;
                if (edgewt == 1) {
                    dq.emplace_back(nx, ny);
                } else {
                    dq.emplace_front(nx, ny);
                }
            }
        }
        return dist[n - 1][m - 1];
    }
};