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
    static bool check(const vector<int>& nums) {
        int n = nums.size();
        bool flip = false;
        for (int i = 1; i < n; ++i) {
            if (nums[i] < nums[i - 1]) {
                if (flip) {
                    return false;
                }
                flip = true;
            }
        }

        return flip ? nums.back() <= nums[0] : true;
    }
};