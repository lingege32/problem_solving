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
    static int minimumSize(vector<int>& nums, long maxOperations) {
        long sum = 0;
        long sz = nums.size();
        for (int num : nums) {
            sum += num;
        }
        int low = max(1L, (sum - sz) / (maxOperations + sz));
        int high = min(1000000000L, (sum - sz) / maxOperations);
        while (low <= high) {
            int mid = (low + high) / 2;
            // find the smallest valid option
            if (canSplit(nums, maxOperations, mid)) {
                high = mid - 1;
            } else {
                low = mid + 1;
            }
        }
        return low;
    }

    // is it possible to split each bag so that each bag has less than or equal to k balls?
    static bool canSplit(const vector<int>& nums, int maxOperations, int k) {
        for (int num : nums) {
            maxOperations -= (num - 1) / k;
        }
        return maxOperations >= 0;
    }
};