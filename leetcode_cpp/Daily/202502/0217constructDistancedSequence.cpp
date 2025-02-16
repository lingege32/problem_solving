#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

constexpr static int V = 1;

class Solution {
    static bool inner(vector<int>& ret, size_t idx, std::vector<std::pair<int8_t, int8_t>>& nums) {
        if (std::ranges::all_of(nums, [](const auto& p) { return p.second == 1; })) {
            return true;
        }
        if (idx == ret.size()) {
            return false;
        }
        if (ret[idx] != V) {
            return inner(ret, idx + 1, nums);
        }
        for (auto& [val, occupy] : nums) {
            if (occupy) {
                continue;
            }
            if (idx + val >= ret.size()) {
                return false;
            }

            auto& left = ret[idx];
            auto& right = ret[idx + val];
            if (left != V || right != V) {
                continue;
            }
            occupy = true;
            left = val;
            right = val;
            if (inner(ret, idx + 1, nums)) {
                return true;
            }
            left = V;
            right = V;
            occupy = false;
        }
        if (ret[idx] == 1) {
            return inner(ret, idx + 1, nums);
        }
        return false;
    }

  public:
    static vector<int> constructDistancedSequence(int n) {
        std::vector<std::pair<int8_t, int8_t>> nums;
        nums.reserve(n);
        for (int i = n; i > 1; --i) {
            nums.emplace_back(i, 0);
        }
        vector<int> ret((2 * n) - 1, V);
        inner(ret, 0, nums);
        return ret;
    }
};