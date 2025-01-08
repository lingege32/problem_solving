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
    static int deleteAndEarn(vector<int>& nums) {
        std::array<int, 10007> dp, zysk;
        dp.fill(0);
        zysk.fill(0);
        static constexpr int N = 10001;
        int s = nums.size();
        for (int i = 0; i < s; i++) {
            zysk[nums[i]] += nums[i];
        }

        dp[0] = 0;
        dp[1] = zysk[1];
        for (int i = 2; i < N; i++) {
            dp[i] = max(dp[i - 1], dp[i - 2] + zysk[i]);
        }
        return dp[N - 1];
    }
};