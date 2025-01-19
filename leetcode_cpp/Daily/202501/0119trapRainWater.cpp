#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

template <typename T, T N = 0>
struct TwoDArray {
    int col = 0;
    std::vector<T> data;

    TwoDArray(int row, int col):
      col(col),
      data(row * col, N) {}

    T* operator[](int row) { return &data[row * col]; }
};

using TwoDArrInt = TwoDArray<int, std::numeric_limits<int>::max()>;
constexpr std::array<int, 4> dx = {0, 0, 1, -1};
constexpr std::array<int, 4> dy = {1, -1, 0, 0};

class Solution {
  public:
    static int trapRainWater(const vector<vector<int>>& heightMap) {
        int m = heightMap.size();
        int n = heightMap[0].size();
        TwoDArrInt dp(m, n);
        std::priority_queue<std::tuple<int, int, int>, std::vector<std::tuple<int, int, int>>, std::greater<>> min_heap;
        if (m > 2) {
            for (int c = 1; c < n - 1; ++c) {
                min_heap.emplace(heightMap[0][c], 1, c);
                min_heap.emplace(heightMap[m - 1][c], m - 2, c);
            }
        }
        if (n > 2) {
            for (int r = 1; r < m - 1; ++r) {
                min_heap.emplace(heightMap[r][0], r, 1);
                min_heap.emplace(heightMap[r][n - 1], r, n - 2);
            }
        }
        auto ifYes = [&](int x, int y) { return x > 0 && x < m - 1 && y > 0 && y < n - 1; };
        while (!min_heap.empty()) {
            auto [cboundry, cx, cy] = min_heap.top();
            min_heap.pop();
            cboundry = std::max(cboundry, heightMap[cx][cy]);
            if (dp[cx][cy] <= cboundry) {
                continue;
            }
            dp[cx][cy] = cboundry;
            for (int i = 0; i < 4; ++i) {
                int nx = cx + dx[i];
                int ny = cy + dy[i];
                if (ifYes(nx, ny)) {
                    min_heap.emplace(cboundry, nx, ny);
                }
            }
        }
        int ret = 0;
        for (int r = 1; r < m - 1; ++r) {
            for (int c = 1; c < n - 1; ++c) {
                if (dp[r][c] != std::numeric_limits<int>::max()) {
                    ret += dp[r][c] - heightMap[r][c];
                }
            }
        }
        return ret;
    }
};
