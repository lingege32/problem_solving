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
    static int minOperations(const vector<int>& nums, int k) {
        auto low_values = nums | views::filter([k](auto num) { return num < k; }) |
                          views::transform([](auto num) { return static_cast<long long>(num); });
        auto q = priority_queue(ranges::begin(low_values), ranges::end(low_values), greater{});
        auto num_ops = 0;
        while (!empty(q)) {
            ++num_ops;
            if (size(q) == 1) {
                break;
            }
            auto x = q.top();
            q.pop();
            auto y = q.top();
            q.pop();
            auto new_val = (min(x, y) << 1) + max(x, y);
            if (new_val < k) {
                q.push(new_val);
            }
        }
        return num_ops;
    }
};