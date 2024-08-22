#include <bits/stdc++.h>
using namespace std;
class Solution {
  public:
    int maxOperations(string s) {
        std::vector<int> cnt;
        size_t idx = 0;
        while (idx != std::string::npos) {
            idx = s.find('1', idx);
            if (idx != std::string::npos) {
                auto idx_0 = s.find('0', idx);
                if (idx_0 != std::string::npos) {
                    cnt.push_back(idx_0 - idx);
                }
                idx = idx_0;
            }
        }
        auto m = cnt.size();
        int ret = 0;
        for (auto c : cnt) {
            ret += (c * m);
            m--;
        }
        return ret;
    }
};