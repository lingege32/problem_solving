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
    static vector<int> getFinalState(vector<int>& nums, int k, int multiplier) {
        std::priority_queue<std::pair<int, int>, std::vector<std::pair<int, int>>, std::greater<>> pq;
        int n = nums.size();
        for (int i = 0; i < n; ++i) {
            pq.emplace(nums[i], i);
        }
        while (k--) {
            auto [val, idx] = pq.top();
            pq.pop();
            val *= multiplier;
            nums[idx] = val;
            pq.emplace(val, idx);
        }
        return std::move(nums);
    }
};