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
    static vector<int> smallestRange(vector<vector<int>>& nums) {
        auto n = nums.size();
        vector<int> idx(n, 0);

        priority_queue<pair<int, int>, vector<pair<int, int>>, greater<>> pq;

        int max = std::numeric_limits<int>::min();
        for (size_t i = 0; i < n; ++i) {
            max = std::max(max, nums[i][0]);
            pq.emplace(nums[i][0], i);
        }
        vector<int> ret = {pq.top().first, max};

        while (pq.size() == n) {
            auto [v, i] = pq.top();
            pq.pop();
            if (max - v < ret[1] - ret[0]) {
                ret = {v, max};
            }
            ;
            if ((++idx[i]) == static_cast<int>(nums[i].size())) {
                break;
            }
            pq.emplace(nums[i][idx[i]], i);
            max = std::max(max, nums[i][idx[i]]);
        }
        return ret;
    }
};