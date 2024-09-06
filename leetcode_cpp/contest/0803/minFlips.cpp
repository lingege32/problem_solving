#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int minFlips(vector<vector<int>>& grid) {
        // 1 make all rows palindromic
        int n = 0;
        for (const auto& row : grid) {
            size_t l = 0;
            size_t r = row.size() - 1;
            while (l < r) {
                if (row[l] != row[r]) {
                    n++;
                }
                l++;
                r--;
            }
        }

        int m = 0;
        size_t col_len = grid.size();
        for (size_t idx = 0; idx < grid[0].size(); ++idx) {
            size_t l = 0;
            size_t r = col_len - 1;
            while (l < r) {
                if (grid[l][idx] != grid[r][idx]) {
                    m++;
                }
                l++;
                r--;
            }
        }
        return std::min(m, n);
    }
};