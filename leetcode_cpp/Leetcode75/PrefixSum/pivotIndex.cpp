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
    static int pivotIndex(vector<int>& nums) {
        int n = nums.size();
        std::vector<int> prefixSum(n, 0);
        int prev = 0;
        for (int i = 1; i < n; ++i) {
            prev += nums[i - 1];
            prefixSum[i] = prev;
        }
        int ans = -1;
        if (prefixSum.back() == 0) {
            ans = n - 1;
        }
        prev = nums.back();
        for (int i = n - 2; i >= 0; --i) {
            if (prev == prefixSum[i]) {
                ans = i;
            }
            prev += nums[i];
        }
        return ans;
    }
};