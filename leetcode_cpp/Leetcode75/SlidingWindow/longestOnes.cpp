#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
    static bool canPlace(const vector<int>& nums, int m, int k) {
        auto how_many_one = nums[m - 1];
        for (int n = m; n < static_cast<int>(nums.size()); ++n) {
            if (how_many_one + k >= m) {
                return true;
            }
            how_many_one = nums[n] - nums[n - m];
        }
        return how_many_one + k >= m;
    }

  public:
    static int longestOnes(vector<int>& nums, int k) {
        for (size_t i = 1; i < nums.size(); ++i) {
            nums[i] += nums[i - 1];
        }
        int l = 1;
        int r = nums.size() + 1;
        while (l < r) {
            int m = l + ((r - l) >> 1);
            if (canPlace(nums, m, k)) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        return l - 1;
    }
};