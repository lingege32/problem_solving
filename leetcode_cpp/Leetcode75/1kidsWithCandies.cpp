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
    static vector<bool> kidsWithCandies(vector<int>& candies, int extraCandies) {
        std::vector<bool> ret(candies.size(), false);
        auto max = *std::ranges::max_element(candies);
        for (size_t i = 0; i < candies.size(); i++) {
            ret[i] = (candies[i] + extraCandies >= max);
        }
        return ret;
    }
};