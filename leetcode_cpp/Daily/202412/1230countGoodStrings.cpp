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
    static int countGoodStrings(int low, int high, int zero, int one) {
        constexpr static int MOD = 1e9 + 7;
        std::vector<int> dp(high + 1, 0);
        dp[0] = 1;

        for (int i = 1; i < high + 1; ++i) {
            int zidx = i - zero;
            if (zidx >= 0) {
                dp[i] = (dp[i] + dp[zidx]) % MOD;
            }
            int oidx = i - one;
            if (oidx >= 0) {
                dp[i] = (dp[i] + dp[oidx]) % MOD;
            }
        }
        int ret = 0;
        for (; low <= high; ++low) {
            ret = (ret + dp[low]) % MOD;
        }
        return ret % MOD;
    }
};