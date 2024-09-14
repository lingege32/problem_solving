#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int longestSubarray(vector<int>& nums) {
        auto max = *std::max_element(nums.begin(), nums.end());
        for (auto& n : nums) {
            if (n != max) {
                n=0;
            }
        }

        auto idx = std::find(nums.begin(), nums.end(), max);
        int ret = 0;
        while (idx != nums.end()) {
            auto next = std::find(idx + 1, nums.end(), 0);
            ret = std::max(ret, static_cast<int>(next - idx));

            idx = std::find(next, nums.end(), max);
        }

        return ret;
    }
};