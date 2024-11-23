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
    static void handleRow(vector<char>& row) {
        int idx = 0;
        int n = row.size();
        int empty = 0;
        int stone = 0;
        for (int i = 0; i < n; ++i) {
            auto c = row[i];
            if (c == '.') {
                empty++;
            } else if (c == '#') {
                stone++;
            } else {
                for (; empty != 0; empty--, idx++) {
                    row[idx] = '.';
                }
                for (; stone != 0; stone--, idx++) {
                    row[idx] = '#';
                }
                idx++;
            }
        }
        for (; empty != 0; empty--, idx++) {
            row[idx] = '.';
        }
        for (; stone != 0; stone--, idx++) {
            row[idx] = '#';
        }
    }

    static vector<vector<char>> rotateTheBox(vector<vector<char>>& box) {
        for (auto& row : box) {
            handleRow(row);
        }

        int row = box.size();
        int col = box[0].size();
        vector<vector<char>> ret(col, vector<char>(row, '0'));
        for (int i = 0; i < row; ++i) {
            for (int j = 0; j < col; ++j) {
                ret[j][row - i - 1] = box[i][j];
            }
        }
        return ret;
    }
};