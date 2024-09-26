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
    static vector<int> frequencySort(vector<int>& nums) {
        std::unordered_map<int, int> count;
        for (auto n : nums) {
            count[n]++;
        }
        std::ranges::sort(nums, [&count](int a, int b) {
            if (count[a] != count[b]) {
                return count[a] < count[b];
            }
            return a > b;
        });
        return std::move(nums);
    }
};