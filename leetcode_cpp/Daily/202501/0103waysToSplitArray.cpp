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
    static int waysToSplitArray(vector<int>& nums) {
        int n = nums.size();
        long total = std::accumulate(nums.begin(), nums.end(), 0L);
        long cur = 0;
        int ret = 0;
        for (int i = 0; i < n - 1; ++i) {
            cur += nums[i];
            total -= nums[i];
            if (cur >= total) {
                ret++;
            }
        }
        return ret;
    }
};