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
    static int maxAscendingSum(vector<int>& nums) {
        int n = nums.size();
        int cur = nums[0];
        int ret = cur;
        for (int i = 1; i < n; ++i) {
            if (nums[i] <= nums[i - 1]) {
                ret = std::max(ret, cur);
                cur = nums[i];
            } else {
                cur += nums[i];
            }
        }
        return std::max(ret, cur);
    }
};