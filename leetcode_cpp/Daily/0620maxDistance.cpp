#include <bits/stdc++.h>
#include <limits>
using namespace std;

class Solution {
  public:
    int maxDistance(vector<int> &position, int m) {
        std::sort(position.begin(), position.end());
        auto right = position.back() + 1;
        int left = 1;
        while (left < right) {
            auto mid = left + (right - left) / 2;
            int con = 0;
            int min = std::numeric_limits<int>::min();
            for (auto p : position) {
                if (p >= min) {
                    con++;
                    min = p + mid;
                }
                if (con == m) {
                    break;
                }
            }
            if (con == m) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return left - 1;
    }
};