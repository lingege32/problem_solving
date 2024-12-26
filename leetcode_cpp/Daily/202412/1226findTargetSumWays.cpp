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
    static int findTargetSumWays(const vector<int>& nums, int target) {
        int n = nums.size();
        int sum = 0;
        for (int i = 0; i < n; i++) {
            sum += nums[i];
        }
        if ((sum + target) % 2 != 0) {
            return 0;
        }
        if (abs(target) > abs(sum)) {
            return 0;
        }
        int new_target = (sum + target) / 2;
        vector<vector<int>> t(n + 1, vector<int>(new_target + 1, 0));
        t[0][0] = 1;
        for (int i = 1; i < n + 1; i++) {
            for (int j = 0; j < new_target + 1; j++) {
                if (nums[i - 1] <= j) {
                    t[i][j] = t[i - 1][j - nums[i - 1]] + t[i - 1][j];
                } else {
                    t[i][j] = t[i - 1][j];
                }
            }
        }
        return t[n][new_target];
    }
};