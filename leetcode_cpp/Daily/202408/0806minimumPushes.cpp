#include <bits/stdc++.h>

using namespace std;

class Solution {
  public:
    static int minimumPushes(const string& word) {
        std::array<std::pair<int, int>, 26> info;
        info.fill({0, 0});
        for (auto c : word) {
            info[c - 'a'].first++;
        }
        std::ranges::sort(info, [](const auto& lhs, const auto& rhs) { return lhs.first > rhs.first; });
        for (size_t i = 0; i < 26; ++i) {
            info[i].second = 1 + (i / 8);
        }

        return std::accumulate(info.begin(), info.end(), 0,
                               [](int acc, const auto& p) { return acc + p.second * p.first; });
    }
};