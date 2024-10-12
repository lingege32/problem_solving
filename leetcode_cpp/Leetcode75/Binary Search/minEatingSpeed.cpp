#include <bits/stdc++.h>

#include <algorithm>
using namespace std;
auto init = []() {
    ios::sync_with_stdio(false);
    cin.tie(nullptr);
    cout.tie(nullptr);
    return 'c';
}();

class Solution {
  public:
    static int minEatingSpeed(vector<int>& piles, int h) {
        auto right = *std::ranges::max_element(piles);
        auto left = 1;
        while (left < right) {
            int m = left + ((right - left) / 2);
            auto hours = [&]() {
                int r = 0;
                for (auto p : piles) {
                    r += ((p / m) + (p % m != 0));
                }
                return r;
            }();
            if (hours <= h) {
                right = m;
            } else {
                left = m + 1;
            }
        }
        return left;
    }
};