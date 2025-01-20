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
    static int firstCompleteIndex(vector<int>& arr, vector<vector<int>>& mat) {
        std::vector<std::pair<int, int>> id2coor(arr.size() + 1);
        int row = mat.size();
        int col = mat[0].size();
        for (int r = 0; r < row; ++r) {
            for (int c = 0; c < col; ++c) {
                id2coor[mat[r][c]] = {r, c};
            }
        }
        std::vector<int> rowCnt(row, 0);
        std::vector<int> colCnt(col, 0);
        int s = arr.size();
        for (int i = 0; i < s; ++i) {
            auto [r, c] = id2coor[arr[i]];
            if (++rowCnt[r] == col || ++colCnt[c] == row) {
                return i;
            }
        }
        std::unreachable();
    }
};