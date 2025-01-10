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
    static int minFallingPathSum(vector<vector<int>>& matrix) {
        int row = matrix.size();
        int col = matrix[0].size();
        for (int r = 1; r < row; ++r) {
            auto& prevr = matrix[r - 1];
            for (int c = 0; c < col; ++c) {
                auto c1 = std::max(0, c - 1);
                auto c2 = std::min(col - 1, c + 1);
                auto min = std::min({prevr[c1], prevr[c2], prevr[c]});
                matrix[r][c] += min;
            }
        }
        return *std::min_element(matrix.back().begin(), matrix.back().end());
    }
};