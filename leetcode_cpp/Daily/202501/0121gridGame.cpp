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
    static long long gridGame(vector<vector<int>>& grid) {
        long long top = std::accumulate(grid[0].begin() + 1, grid[0].end(), 0LL);
        long long bottom = 0;
        int col = grid[0].size();
        long long ret = std::max(top, bottom);
        for (int i = 1; i < col; ++i) {
            top -= grid[0][i];
            bottom += grid[1][i - 1];
            ret = std::min(ret, std::max(top, bottom));
        }
        return ret;
    }
};