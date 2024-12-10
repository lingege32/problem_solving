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
    static int maxTwoEvents(vector<vector<int>>& events) {
        constexpr size_t SIZE = 2e5;
        array<tuple<int, int, int>, SIZE> times;
        size_t pos = 0;
        for (auto& e : events) {
            times[pos++] = {e[0], 1, e[2]};
            times[pos++] = {e[1] + 1, 0, e[2]};
        }
        int res = 0, maxV = 0;
        sort(begin(times), begin(times) + pos);
        for (size_t i = 0; i < pos; i++) {
            auto [time, type, val] = times[i];
            if (type) {
                res = max(res, val + maxV);
            } else {
                maxV = max(maxV, val);
            }
        }
        return res;
    }
};