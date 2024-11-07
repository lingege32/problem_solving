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
    static int countSquares(vector<vector<int>>& matrix) {
        int row = matrix.size();
        int col = matrix[0].size();
        int ret = 0;
        for (int i = 0; i < row; ++i) {
            for (int j = 0; j < col; ++j) {
                ret += countSquares(matrix, i, j);
            }
        }

        return ret;
    }

    static int countSquares(const vector<vector<int>>& matrix, int i, int j) {
        int row = matrix.size();
        int col = matrix[0].size();
        int ret = 0;
        for (int level = 0; i + level < row && j + level < col; level++) {
            for (int x = 0; x <= level; ++x) {
                if (matrix[i + x][j + level] == 0 || matrix[i + level][j + x] == 0) {
                    return ret;
                }
            }
            ret++;
        }

        return ret;
    }
};