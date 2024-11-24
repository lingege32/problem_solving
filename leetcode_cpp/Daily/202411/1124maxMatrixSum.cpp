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
    static long long maxMatrixSum(vector<vector<int>>& matrix) {
        long long ret = 0;
        int min = std::numeric_limits<int>::max();
        int neg_count = 0;
        for (auto& row : matrix) {
            for (auto val : row) {
                if (val < 0) {
                    neg_count++;
                }
                ret += std::abs(val);
                min = std::min(min, std::abs(val));
            }
        }
        return neg_count % 2 == 0 ? ret : ret - (2 * min);
    }
};