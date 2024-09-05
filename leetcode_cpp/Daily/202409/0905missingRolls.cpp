#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static vector<int> missingRolls(vector<int>& rolls, int mean, int n) {
        auto total = mean * (rolls.size() + n);
        auto retain = total - std::accumulate(rolls.begin(), rolls.end(), 0);
        if (retain < static_cast<size_t>(n)) {
            return {};
        }
        auto p = retain / n;
        auto q = retain % n;
        if (p > 6 || (p == 6 && q > 0)) {
            return {};
        }
        std::vector<int> ret(n, p);
        for (auto& r : ret) {
            if (q-- > 0) {
                r++;
            } else {
                break;
            }
        }
        return ret;
    }
};