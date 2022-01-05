#include "header1.h"
#include <algorithm>
#include <memory.h>
#include <string_view>

int Solution::bitwiseComplement(int n) {
    if (n == 0) {
        return 1;
    }
    int ans = 0;
    int mask = 1 << 30;
    while (mask > 0 && (mask & n) == 0) {
        mask >>= 1;
    }
    //   std::cout<<(void*)mask<<", "<<(void*)n<<std::endl;
    while (mask > 0) {
        ans <<= 1;
        if ((mask & n) == 0) {
            ans += 1;
        }
        mask >>= 1;
    }
    //   std::cout<<ans<<std::endl;
    return ans;
}

vector<vector<string>> Solution::partition(string s) {
    struct Solution2 {
        bool dp[20][20];
        vector<string> curList;
        int n;

      public:
        vector<vector<string>> partition(string s) {
            vector<vector<string>> ans;
            n = s.length();

            memset(dp, false, sizeof dp);
            dfs(ans, s, 0);

            return ans;
        }

        void dfs(vector<vector<string>> &ans, string &s, int start) {
            if (start >= n) {
                ans.emplace_back(curList);
            }

            for (int i = start; i < n; i++) {
                if (s[start] == s[i] &&
                    (i - start <= 2 || dp[start + 1][i - 1])) {
                    dp[start][i] = true;
                    curList.emplace_back(s.substr(start, i - start + 1));
                    dfs(ans, s, i + 1);
                    curList.pop_back();
                }
            }
        }
    };
    Solution2 s2;
    return s2.partition(std::move(s));
}

void Solution::solve(vector<vector<char>> &board) {
    struct Solution2 {
      public:
        int go[4][2] = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};

        bool isSafe(int i, int j, int n, int m) {
            return (i > 0 && i < n && j > 0 && j < m);
        }

        void dfs(vector<vector<char>> &a, int i, int j, int n, int m) {
            a[i][j] = 'B';

            for (int k = 0; k < 4; k++) {
                int newi = i + go[k][0];
                int newj = j + go[k][1];

                if (isSafe(newi, newj, n, m) && a[newi][newj] == 'O') {
                    dfs(a, newi, newj, n, m);
                }
            }
        }

        void solve(vector<vector<char>> &a) {
            int n = a.size();
            int m = a[0].size();

            for (int i = 0; i < n; i++) {
                if (a[i][0] == 'O') {
                    dfs(a, i, 0, n, m);
                }
            }
            for (int i = 0; i < n; i++) {
                if (a[i][m - 1] == 'O') {
                    dfs(a, i, m - 1, n, m);
                }
            }
            for (int j = 0; j < m; j++) {
                if (a[0][j] == 'O') {
                    dfs(a, 0, j, n, m);
                }
            }
            for (int j = 0; j < m; j++) {
                if (a[n - 1][j] == 'O') {
                    dfs(a, n - 1, j, n, m);
                }
            }

            for (int i = 0; i < n; i++) {
                for (int j = 0; j < m; j++) {
                    if (a[i][j] == 'O') {
                        a[i][j] = 'X';
                    }
                }
            }
            for (int i = 0; i < n; i++) {
                for (int j = 0; j < m; j++) {
                    if (a[i][j] == 'B') {
                        a[i][j] = 'O';
                    }
                }
            }
        }
    };
    Solution2 s2;
    return s2.solve(board);
}