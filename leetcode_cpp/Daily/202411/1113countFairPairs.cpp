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
    static long long countFairPairs(vector<int>& nums, int lower, int upper) {
        std::ranges::sort(nums);
        return calc(nums, upper + 1) - calc(nums, lower);
    }

    static long long calc(vector<int>& nums, int value) {
        int left = 0, right = nums.size() - 1;
        long long result = 0;
        while (left < right) {
            int sum = nums[left] + nums[right];

            if (sum < value) {
                result += (right - left);
                left++;
            } else {
                right--;
            }
        }

        return result;
    }
};