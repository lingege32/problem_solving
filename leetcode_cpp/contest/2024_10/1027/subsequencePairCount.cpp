#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

using LL = long long;

class Solution {
    static constexpr int MOD = 1e9 + 7;
    std::array<std::array<std::array<int, 201>, 201>, 200> dp;
    std::vector<int>* vals = nullptr;

    int solve(size_t from, int first_gcd, int second_gcd) {
        if (from == vals->size()) {
            return first_gcd && second_gcd && first_gcd == second_gcd;
        }
        auto& v = dp[from][first_gcd][second_gcd];
        if (v != -1) {
            return v;
        }

        LL noTake = solve(from + 1, first_gcd, second_gcd);
        LL takeFirst = solve(from + 1, std::gcd(first_gcd, (*vals)[from]), second_gcd);
        LL takeSecond = solve(from + 1, first_gcd, std::gcd(second_gcd, (*vals)[from]));
        return v = (noTake + takeFirst + takeSecond) % MOD;
    }

  public:
    int subsequencePairCount(vector<int>& nums) {
        std::memset(dp.data(), -1, sizeof(dp));
        vals = &nums;
        return solve(0, 0, 0);
    }
};