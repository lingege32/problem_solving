#include <bits/stdc++.h>
using namespace std;
class Solution {
public:
  int countOfPairs(vector<int> &nums) {
    auto first = nums[0];
    vector<int> dp(first + 1, 1);
    int limit = first;
    for (auto it = nums.begin() + 1; it != nums.end(); ++it) {
      auto sec = *it;
      std::vector<int> sec_dp(sec + 1, 0);
      for (int i = 0; i <= sec; ++i) {
        for (int k = 0; k <= i && k <= limit; ++k) {
          if (limit - k >= sec - i) {
            sec_dp[i] += dp[k];
            sec_dp[i] %= 1000000007;
          }
        }
      }

      std::swap(limit, sec);
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