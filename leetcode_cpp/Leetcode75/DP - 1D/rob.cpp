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
    static int rob(const vector<int>& nums) {
        int take = nums[0];
        int no_take = 0;
        for (size_t idx = 1; idx < nums.size(); ++idx) {
            auto steal = no_take + nums[idx];
            no_take = std::max(take, no_take);
            take = steal;
        }
        return std::max(take, no_take);
    }
};