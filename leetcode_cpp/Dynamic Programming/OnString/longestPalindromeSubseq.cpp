#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

template <size_t N>
struct TwoDArr {
    constexpr static int row = N;
    std::array<int, N * N> data;

    explicit TwoDArr() { data.fill(0); }

    int* operator[](int i) { return &data[i * row]; }
};

class Solution {
  public:
    TwoDArr<1000> memo;

    int longestPalindromeSubseq(const string& s) { return dp(s, 0, s.size() - 1); }

    int dp(const string& s, int l, int r) {
        if (l > r) {
            return 0;  // Return 0 since it's empty string
        }
        if (l == r) {
            return 1;  // Return 1 since it has 1 character
        }
        if (memo[l][r] != 0) {
            return memo[l][r];
        }
        if (s[l] == s[r]) {
            return memo[l][r] = dp(s, l + 1, r - 1) + 2;
        }
        return memo[l][r] = max(dp(s, l + 1, r), dp(s, l, r - 1));
    }
};