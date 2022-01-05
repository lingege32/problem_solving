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

int Solution::uniquePathsIII(vector<vector<int>> &grid) {
    struct Solution2 {
        vector<vector<short>> visit;
        int path;
        int count;
        int current_count;
        int end_r, end_c;

        bool isSafe(vector<vector<int>> &grid, int r, int c, int row, int col) {
            return r >= 0 && c >= 0 && r < row && c < col && grid[r][c] != -1;
        }
        int uniquePathsIII(vector<vector<int>> &grid) {
            int row = grid.size();
            int col = grid[0].size();
            visit = vector<vector<short>>(row, vector<short>(col, 0));
            path = row * col;
            int start_r = 0;
            int start_c = 0;
            end_r = 0;
            end_c = 0;
            count = 0;
            current_count = 0;
            for (size_t r = 0; r < row; ++r) {
                for (size_t c = 0; c < col; ++c) {
                    if (grid[r][c] != 0) {
                        if (grid[r][c] == 1) {
                            start_r = r;
                            start_c = c;
                        } else if (grid[r][c] == -1) {
                            path -= 1;
                        } else {
                            end_r = r;
                            end_c = c;
                        }
                    }
                }
            }
            dfs(grid, start_r, start_c, row, col);
            return count;
        }
        void dfs(vector<vector<int>> &grid, int r, int c, int row, int col) {
            if (visit[r][c] == 1) {
                return;
            }
            if (r == end_r && c == end_c && current_count + 1 == path) {
                count++;
            }
            visit[r][c] = 1;
            current_count++;

            if (isSafe(grid, r - 1, c, row, col)) {
                dfs(grid, r - 1, c, row, col);
            }
            if (isSafe(grid, r + 1, c, row, col)) {
                dfs(grid, r + 1, c, row, col);
            }
            if (isSafe(grid, r, c - 1, row, col)) {
                dfs(grid, r, c - 1, row, col);
            }
            if (isSafe(grid, r, c + 1, row, col)) {
                dfs(grid, r, c + 1, row, col);
            }
            visit[r][c] = 0;
            current_count--;
        }
    };
    Solution2 s2;
    return s2.uniquePathsIII(grid);
}