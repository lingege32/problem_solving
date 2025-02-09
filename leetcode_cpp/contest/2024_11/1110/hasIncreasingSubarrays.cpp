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
    static bool hasIncreasingSubarrays(vector<int>& nums, int k) {
        if (k == 1) {
            return true;
        }
        int n = nums.size();

        int prev = nums[0];
        int prev_idx = 0;
        std::vector<int8_t> dp(n, false);
        for (int i = 1; i < n; ++i) {
            if (nums[i] > prev) {
                if (i - prev_idx + 1 >= k) {
                    dp[i] = true;
                }
            } else {
                prev_idx = i;
            }
            prev = nums[i];
        }
        for (int i = (2 * k) - 1; i < n; ++i) {
            if (dp[i] && dp[i - k]) {
                return true;
            }
        }
        return false;
    }
};