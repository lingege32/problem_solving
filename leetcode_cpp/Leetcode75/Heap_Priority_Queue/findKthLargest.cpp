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
    static int findKthLargest(vector<int>& nums, int k) {
        k -= 1;
        std::nth_element(nums.begin(), nums.begin() + k, nums.end(), [](auto lhs, auto rhs) { return lhs > rhs; });
        return nums[k];
    }
};