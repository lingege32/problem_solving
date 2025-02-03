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
    static int inner(const vector<int>& nums, auto&& F) {
        int cur = 1;
        int ret = 1;
        const int n = nums.size();
        for (int i = 1; i < n; ++i) {
            if (F(nums[i], nums[i - 1])) {
                ret = std::max(cur, ret);
                cur = 1;
            } else {
                cur++;
            }
        }
        return std::max(ret, cur);
    }

    static int longestMonotonicSubarray(vector<int>& nums) {
        return std::max(inner(nums, std::less_equal<>()), inner(nums, std::greater_equal<>()));
    }
};