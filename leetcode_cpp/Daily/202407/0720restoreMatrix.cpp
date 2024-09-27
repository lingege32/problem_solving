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
    static vector<vector<int>> restoreMatrix(vector<int>& rowSum, vector<int>& colSum) {
        int rn = rowSum.size();
        int cn = colSum.size();
        vector<vector<int>> ret(rn, vector<int>(cn, 0));
        for (int r = 0; r < rn; ++r) {
            int rcount = rowSum[r];
            for (int c = 0; c < cn && rcount != 0; ++c) {
                int put = std::min(colSum[c], rcount);
                ret[r][c] = put;
                colSum[c] -= put;
                rcount -= put;
            }
        }
        return ret;
    }
};