#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    int countOfPairs(vector<int> &nums) {
        auto first = nums[0];
        vector<int> dp(first + 1, 1);
        int prev = first;
        for (auto it = nums.begin() + 1; it != nums.end(); ++it) {
            auto cur = *it;
            std::vector<int> sec_dp(cur + 1, 0);
            int cnt = 0;
            int prev_k = 0;
            for (int i = 0; i <= cur; ++i) {
                if (prev_k <= std::min(i, i - (cur - prev))) {
                    cnt = (cnt + dp[prev_k]) % 1000000007;
                    prev_k++;
                }
                sec_dp[i] = cnt;
            }

            std::swap(prev, cur);
            std::swap(sec_dp, dp);
        }
        int ret = 0;
        for (auto d : dp) {
            ret += d;
            ret %= 1000000007;
        }
        return ret;
    }
};