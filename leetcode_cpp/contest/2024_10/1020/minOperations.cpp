#include <bits/stdc++.h>

#include <algorithm>
using namespace std;

std::array<int, 1000001> largestProperDivisor;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);

    for (int i = 1; i < 1000001; i++) {
        for (int j = 2; j * j <= i; j++) {
            if (i % j == 0) {
                largestProperDivisor[i] = max({largestProperDivisor[i], j, i / j});
            }
        }
    }

    return 'c';
}();

class Solution {
  public:
    static int minOperations(vector<int>& nums) {
        int n = nums.size(), operations = 0;
        for (int i = n - 2; i >= 0; i--) {
            while (nums[i] > nums[i + 1]) {
                if (largestProperDivisor[nums[i]] == 0) {
                    return -1;
                }

                nums[i] /= largestProperDivisor[nums[i]];
                ++operations;
            }
        }

        return operations;
    }
};