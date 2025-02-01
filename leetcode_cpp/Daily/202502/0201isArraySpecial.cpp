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
    static bool isArraySpecial(vector<int>& nums) {
        bool parity = nums[0] % 2;
        for (auto n : nums | views::drop(1)) {
            if (n % 2 == parity) {
                return false;
            }
            parity = !parity;
        }
        return true;
    }
};