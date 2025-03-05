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
    static string findDifferentBinaryString(const vector<string>& nums) {
        int n = nums.size();
        int max = (1 << n);
        std::vector<bool> exist(max, false);
        for (auto& num : nums) {
            int v = 1;
            int total = 0;
            for (auto c : num | std::views::reverse) {
                total += (v * (c - '0'));
                v <<= 1;
            }
            exist[total] = true;
        }
        for (int i = 0; i < max; ++i) {
            if (!exist[i]) {
                std::string ret;
                ret.resize(n, '0');
                for (int idx = n - 1; idx >= 0; --idx) {
                    ret[idx] = '0' + (i & 1);
                    i >>= 1;
                }
                return ret;
            }
        }
        std::unreachable();
    }
};