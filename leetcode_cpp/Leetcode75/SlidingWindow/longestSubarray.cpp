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
    static int longestSubarray(vector<int>& nums) {
        int zeros = 0;
        int l = 0;
        int n = nums.size();
        int ans = 0;
        for (int r = 0; r < n; ++r) {
            if (nums[r] == 0) {
                zeros++;
            }
            if (zeros > 1) {
                if (nums[l] == 0) {
                    zeros--;
                }
                l++;
            } else {
                ans = std::max(ans, r - l);
            }
        }
        return ans;
    }
};