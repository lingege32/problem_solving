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
    static bool increasingTriplet(vector<int>& nums) {
        if (nums.size() < 3) {
            return false;
        }
        int one = nums[0];
        int two = std::numeric_limits<int>::max();
        // the second candidate
        int three = nums[0];
        int four = std::numeric_limits<int>::max();
        for (size_t i = 1; i < nums.size(); ++i) {
            auto v = nums[i];
            if (v < three) {
                three = v;
                four = std::numeric_limits<int>::max();
            } else if (v > three && v < four) {
                four = v;
            }

            if (four < two) {
                one = three;
                two = four;
            }
            if (one < two && two < v) {
                return true;
            }
        }
        return false;
    }
};