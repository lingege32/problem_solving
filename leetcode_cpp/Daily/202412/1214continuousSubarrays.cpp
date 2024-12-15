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
    static long long continuousSubarrays(vector<int>& nums) {
        std::map<int, int> mp;
        int left = 0;
        int right = 1;
        mp[nums[0]] = 1;
        int n = nums.size();
        long long ans = 0;
        while (right < n) {
            mp[nums[right]]++;
            auto is_continue = std::abs(mp.begin()->first - mp.rbegin()->first) <= 2;
            if (!is_continue) {
                long long range = right - left;
                ans += (range * (range + 1)) / 2;
                do {
                    auto num = nums[left];
                    auto it = mp.find(num);
                    if (it->second == 1) {
                        mp.erase(it);
                    } else {
                        it->second--;
                    }
                    left++;
                } while (std::abs(mp.begin()->first - mp.rbegin()->first) > 2);
                range = right - left;
                ans -= (range * (range + 1)) / 2;
            }
            right++;
        }

        long long range = right - left;
        ans += (range * (range + 1)) / 2;
        return ans;
    }
};