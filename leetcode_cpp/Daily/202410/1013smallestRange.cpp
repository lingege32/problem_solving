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
        auto k = nums.size();
        auto comp = [](const auto& a, const auto& b) { return a.first > b.first; };
        vector<int> indexes(k, 0);
        priority_queue<pair<int, int>, vector<pair<int, int>>, decltype(comp)> pq(comp);
        int mx = INT_MIN;
        for (size_t i = 0; i < k; i++) {
            pq.emplace(nums[i][0], i);
            mx = max(mx, nums[i][0]);
        }
        vector<int> ans(2);
        ans[0] = pq.top().first;
        ans[1] = mx;
        while (pq.size() == k) {
            pair<int, int> ele = pq.top();
            pq.pop();
            if (mx - ele.first < ans[1] - ans[0]) {
                ans[0] = ele.first;
                ans[1] = mx;
            }
            if (static_cast<size_t>(indexes[ele.second]) < (nums[ele.second].size() - 1)) {
                indexes[ele.second]++;
                pq.emplace(nums[ele.second][indexes[ele.second]], ele.second);
                mx = max(mx, nums[ele.second][indexes[ele.second]]);
            }
        }
        return ans;
    }
};