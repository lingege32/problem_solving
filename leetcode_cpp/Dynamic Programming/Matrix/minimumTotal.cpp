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
    static int minimumTotal(vector<vector<int>>& triangle) {
        for (size_t i = 1; i < triangle.size(); ++i) {
            auto& row = triangle[i];
            auto& prev_row = triangle[i - 1];
            row[0] += prev_row[0];
            row.back() += prev_row.back();
            for (size_t j = 1; j < row.size() - 1; ++j) {
                auto min = std::min(prev_row[j - 1], prev_row[j]);
                row[j] += min;
            }
        }
        return *std::ranges::min_element(triangle.back());
    }
};