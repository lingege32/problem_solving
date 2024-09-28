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
    static double averageWaitingTime(vector<vector<int>>& customers) {
        int start = 0;
        uint64_t ans = 0;
        for (const auto& c : customers) {
            auto ss = c[0];
            auto ww = c[1];
            start = std::max(start, ss) + ww;
            ans += (start - ss);
        }
        return static_cast<double>(ans) / customers.size();
    }
};