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
    static int shortestSubarray(vector<int>& nums, int k) {
        int ret = std::numeric_limits<int>::max();
        long long curSum = 0;
        std::deque<std::pair<long long, int>> q;
        const int n = nums.size();
        for (int i = 0; i < n; ++i) {
            curSum += nums[i];
            if (curSum >= k) {
                ret = std::min(ret, i + 1);
            }

            while (!q.empty() && curSum - q.front().first >= k) {
                ret = std::min(ret, i - q.front().second);
                q.pop_front();
            }

            while (!q.empty() && curSum <= q.back().first) {
                q.pop_back();
            }

            q.emplace_back(curSum, i);
        }
        return ret == std::numeric_limits<int>::max() ? -1 : ret;
    }
};