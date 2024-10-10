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
    static int maxWidthRamp(vector<int>& nums) {
        int n = nums.size();
        std::vector<int> stack;
        stack.push_back(0);
        for (int i = 1; i < n; ++i) {
            if (nums[i] < nums[stack.back()]) {
                stack.push_back(i);
            }
        }

        int ans = 0;
        for (int i = n - 1; i >= 0 && !stack.empty(); --i) {
            while (!stack.empty() && nums[i] >= nums[stack.back()]) {
                ans = std::max(ans, i - stack.back());
                stack.pop_back();
            }
        }
        return ans;
    }
};