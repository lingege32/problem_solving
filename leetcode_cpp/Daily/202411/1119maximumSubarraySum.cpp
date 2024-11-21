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
    static long long maximumSubarraySum(vector<int>& nums, int k) {
        const int n = nums.size();
        int max = *std::ranges::max_element(nums);
        long long  ret = 0;
        long long  tmp = 0;
        int len = 0;
        std::vector<char> visited(max + 1, 0);
        for (int i = 0, r = 0; r < n; ++r) {
            int rv = nums[r];
            while (len == k || visited[rv] == 1) {
                visited[nums[i]] = 0;
                tmp -= nums[i];
                len--;
                ++i;
            }
            tmp += rv;
            visited[rv] = 1;
            len++;
            if (len == k) {
                ret = std::max(tmp, ret);
            }
        }
        return ret;
    }
};