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
    static int findMinArrowShots(vector<vector<int>>& points) {
        std::ranges::sort(points, [](const auto& lhs, const auto& rhs) {
            if (lhs[1] == rhs[1]) {
                return lhs[0] < rhs[0];
            }
            return lhs[1] < rhs[1];
        });

        int ret = 1;
        int bound = points[0][1];
        for (auto& point : points | std::views::drop(1)) {
            auto l = point[0];
            auto r = point[1];
            if (l > bound) {
                ret++;
                bound = std::max(bound, r);
            }
        }
        return ret;
    }
};