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
    static long long countBadPairs(vector<int>& nums) {
        std::unordered_map<int, long long> cnt;
        int n = nums.size();
        for (int i = 0; i < n; ++i) {
            cnt[nums[i] - i]++;
        }
        long long ret = static_cast<long long>(n) * (n - 1) / 2;
        for (auto [num, c] : cnt) {
            ret -= c * (c - 1) / 2;
        }
        return ret;
    }
};