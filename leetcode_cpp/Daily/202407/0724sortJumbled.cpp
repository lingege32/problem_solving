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
    static int mapTo(const std::vector<int>& mapping, int num) {
        int base = 10;
        int ans = mapping[num % 10];
        num /= 10;
        while (num != 0) {
            ans += mapping[(num % 10)] * base;
            base *= 10;
            num /= 10;
        }
        return ans;
    }

    static vector<int> sortJumbled(vector<int>& mapping, vector<int>& nums) {
        std::vector<int> afters;
        std::vector<int> indices;
        afters.resize(nums.size());
        indices.resize(nums.size());
        for (size_t idx = 0; idx < nums.size(); ++idx) {
            indices[idx] = idx;
            afters[idx] = mapTo(mapping, nums[idx]);
        }
        std::ranges::sort(indices, [&afters](int a, int b) {
            auto l = afters[a];
            auto r = afters[b];
            if (l != r) {
                return l < r;
            }
            return a < b;
        });
        for (size_t idx = 0; idx < nums.size(); ++idx) {
            afters[idx] = nums[indices[idx]];
        }
        return afters;
    }
};

class OtherSolution {
  public:
    static int mapTo(const std::vector<int>& mapping, int num) {
        int base = 10;
        int ans = mapping[num % 10];
        num /= 10;
        while (num != 0) {
            ans += mapping[(num % 10)] * base;
            base *= 10;
            num /= 10;
        }
        return ans;
    }

    static vector<int> sortJumbled(vector<int>& mapping, vector<int>& nums) {
        std::vector<uint64_t> vvvv;
        int n = nums.size();
        vvvv.resize(nums.size(), 0);
        for (auto idx = 0; idx < n; ++idx) {
            vvvv[idx] = (static_cast<uint64_t>(mapTo(mapping, nums[idx])) << 32) | idx;
        }
        std::ranges::sort(vvvv);
        constexpr uint64_t mask = (1ULL << 32) - 1;
        for (auto& v : vvvv) {
            v = nums[v & mask];
        }
        for (size_t idx = 0; idx < nums.size(); ++idx) {
            nums[idx] = vvvv[idx];
        }
        return std::move(nums);
    }
};