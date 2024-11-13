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
    static vector<int> getMaximumXor(vector<int>& nums, int maximumBit) {
        int maxNum = (1 << maximumBit) - 1;
        int n = nums.size();
        vector<int> res(n, 0);
        int xorRes = 0;
        for (int i = 0; i < n; i++) {
            xorRes ^= nums[i];
            res[n - 1 - i] = maxNum ^ xorRes;
            // if any num in nums is bigger than 2^maximumBit, we should use the following.
            // res[n - 1 - i] = maxNum ^ (maxNum & xorRes);
        }
        return res;
    }
};