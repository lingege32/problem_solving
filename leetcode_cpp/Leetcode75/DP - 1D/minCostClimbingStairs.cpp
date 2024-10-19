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
    static int minCostClimbingStairs(vector<int>& cost) {
        std::vector<int> dp(cost.size() + 1, std::numeric_limits<int>::max());
        dp[0] = dp[1] = 0;
        for (size_t k = 2; k < dp.size(); ++k) {
            dp[k] = std::min(dp[k - 1] + cost[k - 1], dp[k - 2] + cost[k - 2]);
        }
        return dp.back();
    }
};