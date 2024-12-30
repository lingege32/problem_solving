#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

template <int M, int N>
struct TwoDArr {
    std::array<int, M * N> data;

    int* operator[](int i) { return &data[i * N]; }

    void fill(int v) { data.fill(v); }
};

class Solution {
  public:
    static int numWays(vector<string>& words, const string& target) {
        constexpr int MOD = 1e9 + 7;
        const int m = words[0].size();
        const int n = target.size();
        TwoDArr<26, 1000> freq;
        freq.fill(0);
        for (auto& word : words) {
            for (int i = 0; i < m; i++) {
                freq[word[i] - 'a'][i]++;
            }
        }

        std::array<int, 1001> dp;
        dp.fill(0);
        dp[0] = 1;

        for (int i = 0; i < m; ++i) {
            for (int j = n; j >= 1; --j) {
                auto c = target[j - 1];
                long long fr = freq[c - 'a'][i];
                dp[j] = (dp[j] + dp[j - 1] * fr) % MOD;
            }
        }

        return dp[n];
    }
};
