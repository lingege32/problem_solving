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
    static int minimumDeletions(string s) {
        int n = s.size();
        std::vector<int> dp(n, std::numeric_limits<int>::max());
        dp[0] = 0;
        int b_cnt = s[0] == 'b';
        for (int i = 1; i < n; ++i) {
            if (s[i] == 'b') {
                b_cnt++;
                dp[i] = dp[i - 1];
            } else {
                dp[i] = std::min(b_cnt, dp[i - 1] + 1);
            }
        }
        return dp.back();
    }
};