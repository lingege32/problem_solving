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
    static int maximalSquare(vector<vector<char>>& matrix) {
        int ret = 0;
        int row = matrix.size();
        int col = matrix[0].size();
        std::vector<std::vector<int>> imatrix(row, std::vector<int>(col, 0));
        for (int i = 0; i < row; ++i) {
            for (int j = 0; j < col; ++j) {
                imatrix[i][j] = matrix[i][j] - '0';
                ret = std::max(imatrix[i][j], ret);
            }
        }
        for (int i = 1; i < row; ++i) {
            for (int j = 1; j < col; ++j) {
                if (imatrix[i][j] == 1) {
                    imatrix[i][j] = std::min({imatrix[i - 1][j], imatrix[i][j - 1], imatrix[i - 1][j - 1]}) + 1;
                    ret = std::max(ret, imatrix[i][j]);
                }
            }
        }
        return ret * ret;
    }
};