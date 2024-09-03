#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    int maximumLength(vector<int> &nums) {
        int zero = 0, one = 0;
        for (auto &n : nums) {
            n %= 2;
            if (n == 0) {
                zero++;
            } else {
                one++;
            }
        }
        int max = std::max(zero, one);
        std::vector<std::pair<int, int>> dp{nums.size() + 1,
                                            std::make_pair(0, 0)};
        size_t even_idx = 0;
        size_t odd_idx = 0;
        for (size_t idx = 1; auto num : nums) {
            if (num == 0) {
                dp[idx].first = dp[odd_idx].first + 1;
                dp[idx].second =
                    std::max(dp[idx - 1].first, dp[idx - 1].second);
                even_idx = idx;
            } else {
                dp[idx].first = dp[even_idx].first + 1;
                dp[idx].second =
                    std::max(dp[idx - 1].first, dp[idx - 1].second);
                odd_idx = idx;
            }
            idx++;
        }
        auto m = std::max(dp.back().first, dp.back().second);

        return std::max(max, m);
    }
};