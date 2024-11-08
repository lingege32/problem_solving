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
    static int minimumMountainRemovals(vector<int>& nums) {
        std::vector<int> stack;
        std::vector<int> lhs(nums.size(), 0);
        std::vector<int> rhs(nums.size(), 0);
        auto processN = [&](int n) -> int {
            if (stack.empty() || n > stack.back()) {
                stack.push_back(n);
                return stack.size();
            }
            auto it = std::ranges::lower_bound(stack, n);
            *it = n;
            return std::distance(stack.begin(), it) + 1;
        };
        for (size_t idx = 0; idx < nums.size(); ++idx) {
            auto n = nums[idx];
            lhs[idx] = processN(n);
        }
        stack.clear();
        for (int idx = nums.size() - 1; idx >= 0; --idx) {
            auto n = nums[idx];
            rhs[idx] = processN(n);
        }

        int max = 0;
        for (size_t idx = 0; idx < nums.size(); ++idx) {
            // std::cout << "lhs[" << idx << "] = " << lhs[idx] << ", rhs[" << idx << "] = " << rhs[idx] << std::endl;
            auto l = lhs[idx];
            auto r = rhs[idx];
            if (l > 1 && r > 1) {
                int tmp = lhs[idx] + rhs[idx] - 1;
                max = std::max(tmp, max);
            }
        }
        return nums.size() - max;
    }
};