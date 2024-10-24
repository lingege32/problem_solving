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
    static int eraseOverlapIntervals(vector<vector<int>>& intervals) {
        std::ranges::sort(intervals, [](const auto& a, const auto& b) {
            if (a[1] == b[1]) {
                return a[0] > b[0];
            }
            return a[1] < b[1];
        });
        int ret = 0;
        int bound = std::numeric_limits<int>::min();
        for (auto& interval : intervals) {
            auto left = interval[0];
            auto right = interval[1];
            if (left < bound) {
                ret++;
            } else {
                bound = std::max(bound, right);
            }
        }

        return ret;
    }
};