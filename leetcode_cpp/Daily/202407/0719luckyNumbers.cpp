#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static vector<int> luckyNumbers(vector<vector<int>>& matrix) {
        int r = matrix.size();
        int c = matrix[0].size();
        std::vector<int> ans(c, -1);
        for (int ri = 0; ri < r; ++ri) {
            auto it = std::ranges::min_element(matrix[ri]);
            auto minIndex = std::distance(matrix[ri].begin(), it);
            ans[minIndex] = std::max(*it, ans[minIndex]);
        }

        for (int i = 0; i < c; ++i) {
            auto v = ans[i];
            if (v == -1) {
                continue;
            }
            for (int j = 0; j < r; ++j) {
                if (matrix[j][i] > v) {
                    ans[i] = -1;
                    break;
                }
            }
        }
        auto range = std::ranges::remove(ans, -1);
        ans.erase(range.begin(), range.end());
        return ans;
    }
};