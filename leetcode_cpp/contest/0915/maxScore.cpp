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
    static long long maxScore(const vector<int>& a, const vector<int>& b) {
        int n = b.size();
        std::vector<long long> dp(n);
        for (int i = 0; i < n; ++i) {
            dp[i] = static_cast<long long>(a[0]) * b[i];
        }

        for (int i = 1; i < 4; ++i) {
            std::vector<long long> cur_dp(n, 0);
            long long localMax = dp[i - 1];
            long long base = a[i];
            for (int j = i; j < n; ++j) {
                cur_dp[j] = localMax + base * b[j];
                localMax = std::max(localMax, dp[j]);
            }
            std::swap(dp, cur_dp);
        }

        return *std::max_element(dp.begin() + 3, dp.end());
    }
};