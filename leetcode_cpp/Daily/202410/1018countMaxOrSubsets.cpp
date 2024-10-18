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
    static void backtrack(const vector<int> &nums, size_t index, int currentOR, int maxOR, int &count) {
        if (currentOR == maxOR) {
            count += pow(2, (nums.size() - index));
            return;
        }

        for (size_t i = index; i < nums.size(); ++i) {
            backtrack(nums, i + 1, currentOR | nums[i], maxOR, count);
        }
    }

    static int countMaxOrSubsets(vector<int> &nums) {
        int maxOR = 0;

        // Step 1: Compute the maximum OR
        for (int num : nums) {
            maxOR |= num;
        }

        int count = 0;
        // Step 2: Backtrack to count the subsets
        backtrack(nums, 0, 0, maxOR, count);

        return count;
    }
};