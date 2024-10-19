#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

template <typename... T>
auto add(T... args) {
    int ret = 0;
    auto inner = [&](auto&& arg) {
        ret += arg;
        ret %= 1000000007;
    };
    (inner(args), ...);
    return ret;
}

class Solution {
  public:
    static int numTilings(int n) {
        std::vector<std::tuple<int, int, int>> dp(n + 1, {1, 0, 0});
        dp[1] = {1, 0, 0};
        for (int i = 2; i <= n; ++i) {
            std::get<0>(dp[i]) = add(std::get<0>(dp[i - 2]) + std::get<0>(dp[i - 1]) + std::get<1>(dp[i - 1]) +
                                     std::get<2>(dp[i - 1]));
            std::get<1>(dp[i]) = add(std::get<0>(dp[i - 2]) + std::get<2>(dp[i - 1]));
            std::get<2>(dp[i]) = add(std::get<0>(dp[i - 2]) + std::get<1>(dp[i - 1]));
        }
        return std::get<0>(dp[n]);
    }
};