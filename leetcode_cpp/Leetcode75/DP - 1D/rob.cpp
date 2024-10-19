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
    static int rob(const vector<int>& nums) {
        vector<std::pair<int, int>> dp(nums.size(), {0, 0});
        dp[0] = {nums[0], 0};
        for (size_t idx = 1; idx < nums.size(); ++idx) {
            auto steal = dp[idx - 1].second + nums[idx];
            auto no_steal = std::max(dp[idx - 1].first, dp[idx - 1].second);
            dp[idx] = {steal, no_steal};
        }
        return std::max(dp.back().first, dp.back().second);
    }
};