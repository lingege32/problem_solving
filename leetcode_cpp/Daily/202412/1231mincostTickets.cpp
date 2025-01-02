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
    int mincostTickets(vector<int>& days, vector<int>& costs) {
        cache.resize(days.size(), -1);
        return dp(days, 0, days.size(), 0, costs);
    }

    int dp(const vector<int>& days, int cur, int len, int cur_cost, const vector<int>& costs) {
        if (cur == len) {
            return 0;
        }
        if (cache[cur] != -1) {
            return cache[cur];
        }
        auto min_cost = std::numeric_limits<int>::max();
        for (int i = 0; i < 3; ++i) {
            auto dur = durations[i];
            auto next = std::lower_bound(days.begin() + cur, days.end(), days[cur] + dur);
            auto next_index = std::distance(days.begin(), next);
            auto cost = costs[i];
            min_cost = std::min(min_cost, cost + dp(days, next_index, len, cur_cost + cost, costs));
        }
        cache[cur] = min_cost;
        return min_cost;
    }

  private:
    std::vector<int> cache;
    std::array<int, 3> durations = {1, 7, 30};
};