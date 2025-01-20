#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

constexpr int mod = 1e9 + 7;

class Solution {
  public:
    static int cv(char x) {
        if (x == 'F') {
            return 0;
        }
        if (x == 'W') {
            return 1;
        }
        return 2;
    }

    static int countWinningSequences(const string& s) {
        int dp[2][1005][2][3] = {{{{0}}}};
        int val = cv(s[0]);

        dp[0][0][0][val] = dp[0][0][1][val] = 1;
        dp[0][1][0][(val + 1) % 3] = 1;
        dp[0][1][1][(val + 2) % 3] = 1;

        int n = s.size();

        for (int i = 1; i < n; i++) {
            int u = i % 2;
            int v = (i + 1) % 2;
            val = cv(s[i]);

            // Clear current state
            memset(dp[u], 0, sizeof(dp[u]));

            // Base cases for j = 0
            dp[u][0][0][val] = (dp[v][0][0][(val + 1) % 3] + dp[v][0][0][(val + 2) % 3]) % mod;
            dp[u][0][0][(val + 1) % 3] = (dp[v][1][1][val] + dp[v][1][1][(val + 2) % 3]) % mod;
            dp[u][0][0][(val + 2) % 3] = (dp[v][1][0][val] + dp[v][1][0][(val + 1) % 3]) % mod;

            dp[u][0][1][val] = (dp[v][0][1][(val + 1) % 3] + dp[v][0][1][(val + 2) % 3]) % mod;
            dp[u][0][1][(val + 1) % 3] = (dp[v][1][1][val] + dp[v][1][1][(val + 2) % 3]) % mod;
            dp[u][0][1][(val + 2) % 3] = (dp[v][1][0][val] + dp[v][1][0][(val + 1) % 3]) % mod;

            // Cases for j > 0
            for (int j = 1; j <= i + 1; j++) {
                dp[u][j][0][val] = (dp[v][j][0][(val + 2) % 3] + dp[v][j][0][(val + 1) % 3]) % mod;
                dp[u][j][0][(val + 1) % 3] = (dp[v][j - 1][0][(val + 2) % 3] + dp[v][j - 1][0][val]) % mod;
                dp[u][j][0][(val + 2) % 3] = (dp[v][j + 1][0][(val + 1) % 3] + dp[v][j + 1][0][val]) % mod;

                dp[u][j][1][val] = (dp[v][j][1][(val + 2) % 3] + dp[v][j][1][(val + 1) % 3]) % mod;
                dp[u][j][1][(val + 1) % 3] = (dp[v][j + 1][1][(val + 2) % 3] + dp[v][j + 1][1][val]) % mod;
                dp[u][j][1][(val + 2) % 3] = (dp[v][j - 1][1][val] + dp[v][j - 1][1][(val + 1) % 3]) % mod;
            }
        }

        // Final calculation of the answer
        int ans = 0;
        int final_state = (n - 1) % 2;
        for (int y = 1; y <= n; y++) {
            ans = (ans + dp[final_state][y][0][0]) % mod;
            ans = (ans + dp[final_state][y][0][1]) % mod;
            ans = (ans + dp[final_state][y][0][2]) % mod;
        }
        return ans;
    }
};
