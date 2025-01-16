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
    static int xorAll(const vector<int>& nums) {
        return std::accumulate(nums.begin(), nums.end(), 0, std::bit_xor<>());
    }

    static int xorAllNums(vector<int>& nums1, vector<int>& nums2) {
        int ret = 0;
        if (nums1.size() % 2 == 1) {
            ret ^= xorAll(nums2);
        }
        if (nums2.size() % 2 == 1) {
            ret ^= xorAll(nums1);
        }
        return ret;
    }
};