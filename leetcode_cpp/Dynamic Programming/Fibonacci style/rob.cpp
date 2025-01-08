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
    static int rob(vector<int>& nums) {
        int n = nums.size();
        std::vector<std::pair<int, int>> dp(n, {0, 0});
        dp[0].first = nums[0];
        for (int i = 1; i < n; ++i) {
            dp[i].first = dp[i - 1].second + nums[i];
            dp[i].second = std::max(dp[i - 1].first, dp[i - 1].second);
        }
        return std::max(dp.back().first, dp.back().second);
    }
};