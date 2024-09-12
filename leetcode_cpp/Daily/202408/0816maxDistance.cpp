#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int maxDistance(vector<vector<int>>& arrays) {
        int res = 0;
        int min = arrays[0].front();
        int max = arrays[0].back();
        for (size_t j = 1; j < arrays.size(); ++j) {
            auto& b = arrays[j];
            auto b1 = b.front();
            auto b2 = b.back();
            auto d1 = std::max(std::abs(min - b1), std::abs(min - b2));
            auto d2 = std::max(std::abs(max - b1), std::abs(max - b2));
            auto d = std::max(d1, d2);
            max = std::max(max, b2);
            min = std::min(min, b1);
            res = std::max(d, res);
        }
        return res;
    }
};