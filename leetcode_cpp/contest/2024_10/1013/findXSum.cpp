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
    static vector<int> findXSum(vector<int>& nums, int k, int x) {
        std::array<int, 51> cnt{};
        int n = nums.size();
        cnt.fill(0);
        std::unordered_set<int> v;
        for (int i = 0; i < k - 1; ++i) {
            cnt[nums[i]]++;
            v.insert(nums[i]);
        }
        std::vector<int> ret;
        ret.reserve(nums.size() - k + 1);
        for (int i = k - 1; i < n; ++i) {
            cnt[nums[i]]++;
            v.insert(nums[i]);
            std::vector<std::pair<int, int>> ans;
            ans.reserve(v.size());
            for (auto val : v) {
                ans.emplace_back(cnt[val], val);
            };
            if (static_cast<int>(ans.size()) > x) {
                std::nth_element(ans.begin(), ans.begin() + x, ans.end(), std::greater<>());
            }
            auto& r = ret.emplace_back(0);
            for (auto [c, val] : ans | std::views::take(x)) {
                r += c * val;
            }
            int remove = i - (k - 1);
            if (cnt[nums[remove]]-- == 1) {
                v.erase(nums[remove]);
            }
        }
        return ret;
    }
};