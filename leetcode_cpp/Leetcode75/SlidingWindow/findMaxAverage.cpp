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
    static double findMaxAverage(vector<int>& nums, int k) {
        int total = 0;
        for (int i = 0; i < k; ++i) {
            total += nums[i];
        }
        int max = total;
        for (int i = k; i < static_cast<int>(nums.size()); ++i) {
            total += (nums[i] - nums[i - k]);
            max = std::max(total, max);
        }
        return max / static_cast<double>(k);
    }
};