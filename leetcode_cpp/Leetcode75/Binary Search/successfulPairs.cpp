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
    static vector<int> successfulPairs(vector<int>& spells, vector<int>& potions, long long success) {
        std::ranges::sort(potions);
        std::vector<int> ret;
        ret.reserve(spells.size());
        for (auto spell : spells) {
            long long least = (success / spell) + (success % spell != 0);
            if (least > std::numeric_limits<int>::max()) {
                ret.push_back(0);
                continue;
            }
            auto lb = std::ranges::lower_bound(potions, least);
            ret.push_back(std::distance(lb, potions.end()));
        }
        return ret;
    }
};