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
    static int maximumBeauty(vector<int>& nums, int k) {
        std::ranges::sort(nums);
        int left = 0, right = 1;
        int n = nums.size();
        int max = 1;
        k *= 2;
        while (right < n) {
            while (nums[left] + k < nums[right]) {
                left++;
            }
            max = std::max(max, right - left + 1);
            right++;
        }
        return max;
    }
};

class OptSolution {
  public:
    static int maximumBeauty(vector<int>& nums, int k) {
        k*=2;
        int m = *std::ranges::max_element(nums) + k + 2;
        vector<int> d(m);
        for (int x : nums) {
            d[x]++;
            d[x + k + 1]--;
        }
        int ans = 0, s = 0;
        for (int x : d) {
            s += x;
            ans = max(ans, s);
        }
        return ans;
    }
};