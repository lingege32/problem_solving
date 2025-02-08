#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

template <typename T>
struct TwoDArray {
    int col = 0;
    std::vector<T> data;

    TwoDArray(int row, int col):
      col(col),
      data(row * col, std::numeric_limits<int>::max()) {}

    T* operator[](int row) { return &data[row * col]; }

    [[nodiscard]] T back() const { return data.back(); }
};

constexpr std::array<int, 5> dis = {0, 1, 0, -1, 0};

struct Cmp {
    bool operator()(const std::tuple<int, int, int>& a, const std::tuple<int, int, int>& b) const {
        return std::get<2>(a) > std::get<2>(b);
    }
};

using Tuple = std::tuple<int, int, int, int>;
using MinHeap = std::priority_queue<Tuple, std::vector<Tuple>, std::greater<>>;

class Solution {
  public:
    static int minTimeToReach(vector<vector<int>>& moveTime) {
        moveTime[0][0] = -2;
        int row = moveTime.size();
        int col = moveTime[0].size();
        TwoDArray<int> dp(row, col);
        MinHeap q;
        q.emplace(0, 2, 0, 0);

        while (!q.empty()) {
            auto [t, move, x, y] = q.top();
            q.pop();
            if (x == row - 1 && y == col - 1) {
                return std::max(t, move + moveTime[x][y]);
            }
            if (dp[x][y] <= t) {
                continue;
            }
            t = std::max(t, move + moveTime[x][y]);
            move ^= 3;
            dp[x][y] = t;
            t += move;
            for (auto i = 1; i <= 4; ++i) {
                int nx = x + dis[i - 1];
                int ny = y + dis[i];
                if (nx >= 0 && nx < row && ny >= 0 && ny < col) {
                    q.emplace(t, move, nx, ny);
                }
            }
        }
        return dp.back();
    }
};