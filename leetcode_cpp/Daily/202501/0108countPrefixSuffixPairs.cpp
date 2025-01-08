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
    static bool isvalid(std::string_view prev, std::string_view next) {
        int ps = prev.size();
        int ns = next.size();
        if (ps > ns) {
            return false;
        }
        bool isValid = true;
        for (int i = 0, j = 0, k = ns - ps; i < ps && isValid; ++i, ++j, ++k) {
            isValid &= prev[i] == next[k];
            isValid &= prev[i] == next[j];
        }
        return isValid;
    }

    static int countPrefixSuffixPairs(const vector<string>& words) {
        int n = 0;
        int s = words.size();
        for (auto i = 0; i < s; ++i) {
            auto& prev = words[i];
            for (auto j = i + 1; j < s; ++j) {
                auto& next = words[j];
                if (isvalid(prev, next)) {
                    n++;
                }
            }
        }
        return n;
    }
};