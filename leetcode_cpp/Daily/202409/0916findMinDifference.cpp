#include <bits/stdc++.h>
using namespace std;

class Solution {
  public:
    static int findMinDifference(vector<string>& timePoints) {
        std::vector<int> times;
        times.reserve(timePoints.size());
        for (const auto& t : timePoints) {
            times.push_back((t[0] - '0') * 600 + (t[1] - '0') * 60 + (t[3] - '0') * 10 + t[4] - '0');
        }
        std::sort(times.begin(), times.end());
        int ret = times[0] + 24 * 60 - times.back();
        for (size_t idx = 1; idx < times.size(); ++idx) {
            ret = std::min(ret, times[idx] - times[idx - 1]);
        }
        return ret;
    }
};