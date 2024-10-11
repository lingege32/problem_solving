#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

typedef pair<int, int> pii;

class Solution {
  public:
    static int smallestChair(vector<vector<int>>& times, int targetFriend) {
        int n = times.size();
        priority_queue<int, vector<int>, greater<>> idle;
        priority_queue<pii, vector<pii>, greater<>> busy;  //[release time, chair id]

        for (int i = 0; i < n; ++i) {
            idle.push(i);
        }
        int target_start = times[targetFriend][0];
        std::ranges::sort(times, [](const auto& lhs, const auto& rhs) { return lhs[0] < rhs[0]; });

        for (const auto& time : times) {
            auto start = time[0];
            auto end = time[1];
            while (!busy.empty() && busy.top().first <= start) {
                idle.push(busy.top().second);
                busy.pop();
            }
            if (start == target_start) {
                return idle.top();
            }
            auto c = idle.top();
            idle.pop();
            busy.emplace(end, c);
        }

        std::unreachable();
    }
};