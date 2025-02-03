#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
    const int mod = 1e9 + 7;
    std::array<std::array<std::array<int,201>,201>,200> dp;

  public:
    int solve(size_t i, vector<int>& nums, int first, int second) {
        // first == gcd of first subsequence till now
        // second = gcd of second subsequence till now

        if (i == nums.size()) {
            return (first && second) && (first == second);
        }

        if (dp[i][first][second] != -1) {
            return dp[i][first][second];
        }

        // Dont include this element in any subsequence
        int skip = solve(i + 1, nums, first, second);

        // Include this index in the first subsequence
        int take1 = solve(i + 1, nums, __gcd(first, nums[i]), second);

        // Include this index in the second subsequence
        int take2 = solve(i + 1, nums, first, __gcd(second, nums[i]));

        // Summing up all the possibilites
        return dp[i][first][second] = (0LL + skip + take1 + take2) % mod;
    }

    int subsequencePairCount(vector<int>& nums) {
        memset(dp.data(), -1, sizeof(dp));
        return solve(0, nums, 0, 0);
    }
};