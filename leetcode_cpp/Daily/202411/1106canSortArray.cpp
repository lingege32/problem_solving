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
    static bool canSortArray(vector<int>& nums) {
        auto prev_min = std::numeric_limits<int>::min();
        auto prev = calSetBit(nums[0]);
        auto be = nums.begin();
        auto find_next = [&]() {
            return std::find_if(be + 1, nums.end(), [&](int n) { return prev != calSetBit(n); });
        };
        auto it = find_next();
        for (; it != nums.end();) {
            auto [min, max] = std::minmax_element(be, it);
            if (prev_min > *min) {
                return false;
            }
            prev_min = *max;
            be = it;
            prev = calSetBit(*it);
            it = find_next();
        }
        auto min = std::min_element(be, it);
        return *min >= prev_min;
    }

    static int calSetBit(int n) {
        int cnt = 0;
        while (n) {
            n &= (n - 1);
            cnt++;
        }
        return cnt;
    }
};