#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static int maxIncreasingSubarrays(vector<int>& nums) {
        int n = nums.size();
        int prev = 0;
        int curr = 1;
        int ans = 1;

        for (int i = 1; i < n; i++) {
            if (nums[i - 1] < nums[i]) {
                curr++;
            } else {
                prev = curr;
                curr = 1;
            }
            ans = max({ans, curr / 2, min(prev, curr)});
        }
        return ans;
    }
};