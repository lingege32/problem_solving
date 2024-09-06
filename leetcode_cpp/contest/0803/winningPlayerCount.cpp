#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int winningPlayerCount(int n, vector<vector<int>>& pick) {
        std::vector<std::array<int, 11>> dp(n, std::array<int, 11>{0});

        for (const auto& p : pick) {
            dp[p[0]][p[1]]++;
        }
        int i = 0;
        for (size_t idx = 0; idx < dp.size(); ++idx) {
            if (std::any_of(dp[idx].begin(), dp[idx].end(), [&](int i) { return static_cast<size_t>(i) > idx; })) {
                i++;
            }
        }
        return i;
    }
};