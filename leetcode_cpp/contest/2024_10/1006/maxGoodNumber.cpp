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
    static int countBits(int n) {
        int ret = 0;
        while (n > 0) {
            ret++;
            n >>= 1;
        }
        return ret;
    }

    static int maxGoodNumber(vector<int>& nums) {
        for (auto& num : nums) {
            num |= (countBits(num) << 16);
        }
        std::ranges::sort(nums);
        int ret = std::numeric_limits<int>::min();
        do {
            int shift = 0;
            int cur = 0;
            for (auto num : nums | std::views::reverse) {
                auto shiftCount = num >> 16;
                auto n = num & 0xffff;
                cur |= (n << shift);
                shift += shiftCount;
            }
            ret = std::max(ret, cur);

        } while (std::next_permutation(nums.begin(), nums.end()));
        return ret;
    }
};