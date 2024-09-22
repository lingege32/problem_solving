#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int minSwaps(vector<int>& nums) {
        int count = std::count(nums.begin(), nums.end(), 1);
        int n = nums.size();
        nums.resize(n + count);
        for (int i = 0; i < count; ++i) {
            nums[i + n] = nums[i];
        }
        int ones = std::count(nums.begin(), nums.begin() + count, 1);
        int max_ones = ones;
        for (int i = count; i < static_cast<int>(nums.size()); ++i) {
            ones += nums[i];
            ones -= nums[i - count];
            max_ones = std::max(max_ones, ones);
        }
        return std::max(0, count - max_ones);
    }
};