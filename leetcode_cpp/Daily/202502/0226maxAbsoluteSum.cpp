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
    static int maxAbsoluteSum(vector<int>& nums) {
        int preSum = 0;
        int minPreSum = 0;
        int maxPreSum = 0;
        int ret = std::numeric_limits<int>::min();
        for (auto num : nums) {
            preSum += num;
            ret = std::max({ret, std::abs(preSum - minPreSum), std::abs(preSum - maxPreSum)});
            minPreSum = std::min(preSum, minPreSum);
            maxPreSum = std::max(preSum, maxPreSum);
        }
        return ret;
    }
};