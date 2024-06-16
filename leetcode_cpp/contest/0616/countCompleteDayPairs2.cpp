#include <unordered_map>

#include <iostream>
#include <vector>
using namespace std;

class Solution {
  public:
    long long countCompleteDayPairs(vector<int> &hours) {
        for (auto &h : hours) {
            h %= 24;
        }
        int complete = 0;
        std::unordered_map<int, int> half;
        for (auto h : hours) {
            if (h % 24 == 0) {
                complete++;
            } else {
                half[h]++;
            }
        }
        long long count = static_cast<long long>(complete) * static_cast<long long>((complete - 1)) / 2;
        // std::cout << "count: " << count << "\n";
        long long extra = 0;
        for (auto [h, c] : half) {
            auto f = 24 - h;
            if (f == h) {
                count += (c * (c - 1) / 2);
                continue;
            }
            auto it = half.find(f);
            if (it != half.end()) {
                extra += (c * it->second);
            }
        }

        return count + extra / 2;
    }
};