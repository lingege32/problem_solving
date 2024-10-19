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
    static int uniquePaths(int m, int n) {
        std::vector<vector<int>> dp(m, vector<int>(n, 0));
        for (int k = 0; k < m; ++k) {
            dp[k][0] = 1;
        }
        for (int k = 0; k < n; ++k) {
            dp[0][k] = 1;
        }

        for (int i = 1; i < m; ++i) {
            for (int j = 1; j < n; ++j) {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        return dp.back().back();
    }
};