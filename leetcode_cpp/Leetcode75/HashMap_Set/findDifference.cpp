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
    static vector<vector<int>> findDifference(vector<int>& nums1, vector<int>& nums2) {
        std::ranges::sort(nums1);
        std::ranges::sort(nums2);
        auto [f1, f2] = std::ranges::unique(nums1);
        nums1.erase(f1, f2);
        auto [f3, f4] = std::ranges::unique(nums2);
        nums2.erase(f3, f4);
        std::unordered_map<int, int> mapping;
        for (auto n : nums1) {
            mapping[n]++;
        }
        for (auto n : nums2) {
            mapping[n]++;
        }

        auto erase = [&](vector<int>& nums) {
            auto r = std::ranges::remove_if(nums, [&](int n) { return mapping[n] == 2; });
            nums.erase(r.begin(), r.end());
        };
        erase(nums1);
        erase(nums2);

        return {std::move(nums1), std::move(nums2)};
    }
};