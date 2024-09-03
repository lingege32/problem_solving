#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    int maximumLength(vector<int> &nums, int k) {
        for (auto &n : nums) {
            n %= k;
        }
        int ret = std::numeric_limits<int>::min();
        auto table = prepareTable(nums, k);
        for (int i = 0; i < k; ++i) {
            ret = std::max(ret, inner(nums, k, i, table));
        }
        return ret;
    }
    std::vector<std::vector<int>> prepareTable(vector<int> &nums, int k) {
        std::vector<std::vector<int>> table;
        table.reserve(nums.size() + 1);
        std::vector<int> cur(k, 0);
        table.emplace_back(cur);
        for (int idx = 1; auto n : nums) {
            cur[n % k] = idx;
            table.emplace_back(cur);
            idx++;
        }
        return table;
    }
    int inner(vector<int> &nums, int k, int r,
              const std::vector<std::vector<int>> &table) {
        std::vector<std::pair<int, int>> dp{nums.size() + 1,
                                            std::make_pair(0, 0)};
        for (size_t idx = 1; auto n : nums) {
            // have n, find a find to make (n + find) % k == r
            int find = (r - n + k) % k;
            auto pre = table[idx-1][find];
            dp[idx].first = dp[pre].first + 1;
            dp[idx].second = std::max(dp[idx - 1].first, dp[idx - 1].second);
            idx++;
        }
        return std::max(dp.back().first, dp.back().second);
    }
};