#include <bits/stdc++.h>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();
using LL = long long;

class Solution {
  public:
    static LL cal(auto be, auto end) {
        LL gcd = *(be++);
        LL lcm = gcd;
        for (; be != end; ++be) {
            LL v = *be;
            gcd = __gcd(gcd, v);
            lcm = (lcm * v) / __gcd(lcm, v);
        }
        return gcd * lcm;
    }

    static LL maxScore(vector<int>& nums) {
        int n = nums.size();
        if (n == 1) {
            return nums[0] * nums[0];
        }
        if (n == 2) {
            auto big = std::max(nums[0], nums[1]);
            return big * big;
        }
        LL max = std::max(cal(nums.begin(), nums.end()), cal(nums.begin() + 1, nums.end()));
        for (int i = 1; i < n; ++i) {
            std::swap(nums[0], nums[i]);
            max = std::max(max, cal(nums.begin() + 1, nums.end()));
            std::swap(nums[0], nums[i]);
        }
        return max;
    }
};