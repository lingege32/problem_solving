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
    static void moveZeroes(vector<int>& nums) {
        auto it = std::ranges::remove(nums, 0);
        std::fill(it.begin(), nums.end(), 0);
    }
};