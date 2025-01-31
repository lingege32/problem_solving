#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
constexpr std::array<int, 4> dx = {0, 0, 1, -1};
constexpr std::array<int, 4> dy = {1, -1, 0, 0};

class Solution {
    int m = 0;

    std::vector<int> areas;

  public:
    void dfs(int x, int y, int& v, int mark, std::vector<std::vector<int>>& graph) {
        if (x < 0 || y < 0 || x >= m || y >= m || graph[x][y] != 1) {
            return;
        }
        graph[x][y] = mark;
        v++;
        for (int i = 0; i < 4; i++) {
            dfs(x + dx[i], y + dy[i], v, mark, graph);
        }
    }

    int largestIsland(vector<vector<int>>& grid) {
        m = grid.size();
        int ret = 0;
        for (int i = 0; i < m; ++i) {
            for (int y = 0; y < m; ++y) {
                int area = 0;
                dfs(i, y, area, areas.size() + 2, grid);
                ret = std::max(ret, area - 1);
                areas.push_back(area);
            }
        }
        for (int i = 0; i < m; ++i) {
            for (int y = 0; y < m; ++y) {
                if (grid[i][y] != 0) {
                    continue;
                }
                std::set<int> neighbormark;
                for (int d = 0; d < 4; ++d) {
                    auto nx = i + dx[d];
                    auto ny = y + dy[d];
                    if (nx < 0 || ny < 0 || nx >= m || ny >= m || grid[nx][ny] == 0) {
                        continue;
                    }
                    neighbormark.insert(grid[nx][ny]);
                }
                ret = std::max(ret, std::accumulate(neighbormark.begin(), neighbormark.end(), 0,
                                                    [&](int acc, int mark) { return acc + areas[mark - 2]; }));
            }
        }
        return ret + 1;
    }
};