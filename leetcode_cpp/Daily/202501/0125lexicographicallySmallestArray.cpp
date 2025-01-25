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
    static vector<int> lexicographicallySmallestArray(vector<int>& nums, int limit) {
        int n = nums.size();
        std::vector<std::pair<int, int>> idxs(n);
        for (int i = 0; i < n; ++i) {
            idxs[i] = make_pair(nums[i], i);
        }
        std::ranges::sort(idxs);
        int left = 0;
        while (left != n) {
            int right = left + 1;
            for (; right < n && idxs[right].first - idxs[right-1].first <= limit; ++right) {
            }
            std::vector<int> tmp;
            tmp.reserve(right - left);
            for (int l = left; l < right; ++l) {
                tmp.emplace_back(idxs[l].second);
            }
            std::ranges::sort(tmp);
            for (int l = 0; left < right; ++left, ++l) {
                nums[tmp[l]] = idxs[left].first;
            }
        }

        return std::move(nums);
    }
};