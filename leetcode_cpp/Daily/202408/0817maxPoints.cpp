#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static long long maxPoints(vector<vector<int>>& points) {
        int n = points[0].size();
        int m = points.size();
        vector<vector<long long>> dp(2, vector<long long>(n, 0));
        for (int i = 0; i < n; ++i) {
            dp[0][i] = points[0][i];
        }
        for (int r = 1; r < m; ++r) {
            long long max = 0;
            for (int c = 0; c < n; ++c) {
                max = std::max(max - 1, dp[0][c]);
                dp[1][c] = max;
            }
            max = 0;
            for (int c = n - 1; c >= 0; --c) {
                max = std::max(max - 1, dp[0][c]);
                dp[1][c] = std::max(dp[1][c], max) + points[r][c];
            }
            std::swap(dp[0], dp[1]);
        }
        return *std::max_element(dp[0].begin(), dp[0].end());
    }
};

static const auto speedup = []() {
    std::ios::sync_with_stdio(false);
    std::cin.tie(nullptr);
    std::cout.tie(nullptr);
    return 0;
}();
