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
    static vector<int> resultsArray(vector<int>& nums, int k) {
        std::vector<int> ret;

        ret.reserve(nums.size() - k + 1);
        if (k == 1) {
            ret = std::move(nums);
            return ret;
        }
        int idx = k - 1;
        int prev = -1;
        for (int i = 1; i < idx; ++i) {
            if (nums[i] - nums[i - 1] != 1) {
                prev = i;
            }
        }
        const int n = nums.size();

        for (; idx < n; ++idx) {
            int v = nums[idx];
            if (v - nums[idx - 1] != 1) {
                prev = idx;
            }
            ret.push_back((idx - prev + 1 >= k) ? v : -1);
        }
        return ret;
    }
};