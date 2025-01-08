#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

struct TwoDArr {
    std::vector<int> data;
    int col = 0;

    TwoDArr(int m, int n):
      data(m * n, 1),
      col{n} {}

    int* operator[](int i) { return &data[i * col]; }

    [[nodiscard]] int back() const { return data.back(); }
};

class Solution {
  public:
    static int uniquePaths(int m, int n) {
        TwoDArr dp{m, n};
        for (int i = 1; i < m; ++i) {
            for (int j = 1; j < n; ++j) {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        return dp.back();
    }
};