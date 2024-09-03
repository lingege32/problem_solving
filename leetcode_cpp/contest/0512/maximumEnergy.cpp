#include <bits/stdc++.h>
using namespace std;
class Solution {
  public:
    int maximumEnergy(vector<int> &energy, int k) {
        size_t kk = k;
        auto s = energy.size();
        if (s <= kk) {
            return *std::max_element(energy.begin(), energy.end());
        }
        int max = *std::max_element(energy.rbegin(), energy.rbegin() + kk);
        for (int idx = s - kk - 1; idx >= 0; --idx) {
            auto &e = energy[idx];
            e += energy[idx + kk];
            max = std::max(e, max);
        }
        return max;
    }
};