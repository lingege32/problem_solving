#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    double minimumAverage(vector<int>& nums) {
        std::sort(nums.begin(), nums.end());
        size_t left = 0;
        size_t right = nums.size()-1;
        int min = std::numeric_limits<int>::max();
        while (left < right) {
            min = std::min(min, nums[left] + nums[right]);
            left++;
            right--;
        }
        return static_cast<double>(min) / 2;
    }
};